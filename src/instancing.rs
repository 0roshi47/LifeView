use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        query::QueryItem,
        system::{SystemParamItem, lifetimeless::*},
    },
    math::FloatOrd,
    mesh::{MeshVertexBufferLayoutRef, VertexBufferLayout},
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_phase::*,
        render_resource::*,
        renderer::RenderDevice,
        sync_world::MainEntity,
        view::ExtractedView,
    },
    sprite_render::{
        Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup, init_mesh_2d_pipeline,
    }, // sprite::{Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances, SetMesh2dBindGroup, SetMesh2dViewBindGroup},
};
use bytemuck::{Pod, Zeroable};

use crate::cell::Cell;
use crate::grid::Grid;
use crate::grid_coloration::ColorGradient;

const BASE_CELL_WIDTH: usize = 75;

// --- Per-instance data sent to GPU ---
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct CellInstance {
    pub position: Vec2, // world position
    pub cell_size: f32,
    pub state: f32,
}

// --- Component holding all instances ---
#[derive(Component, Deref)]
pub struct CellInstanceData(pub Vec<CellInstance>);

impl ExtractComponent for CellInstanceData {
    type QueryData = &'static CellInstanceData;
    type QueryFilter = ();
    type Out = Self;
    fn extract_component(item: QueryItem<'_, '_, Self::QueryData>) -> Option<Self> {
        Some(CellInstanceData(item.0.clone()))
    }
}

// --- Marker component to find the grid entity ---
#[derive(Component)]
pub struct CellGrid;

// --- Component holding current gradient index (extracted to render world) ---
#[derive(Component, Default, Deref, DerefMut)]
pub struct GradientIndex(pub usize);

impl ExtractComponent for GradientIndex {
    type QueryData = &'static GradientIndex;
    type QueryFilter = ();
    type Out = Self;
    fn extract_component(item: QueryItem<'_, '_, Self::QueryData>) -> Option<Self> {
        Some(GradientIndex(item.0))
    }
}

pub struct CellMaterialPlugin;
impl Plugin for CellMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<CellInstanceData>::default());
        app.add_plugins(ExtractComponentPlugin::<GradientIndex>::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, rebuild_grid_instances);
        app.add_systems(FixedUpdate, update_instance_data);

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawCells>()
            .init_resource::<SpecializedMeshPipelines<CellPipeline>>()
            .add_systems(
                RenderStartup,
                init_cell_pipeline.after(init_mesh_2d_pipeline),
            )
            .add_systems(
                Render,
                (
                    queue_cells.in_set(RenderSystems::QueueMeshes),
                    prepare_cell_buffers.in_set(RenderSystems::PrepareResources),
                ),
            );
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, windows: Query<&Window>) {
    commands.spawn(Camera2d);

    let window = windows.single().unwrap();
    let cell_size = window.resolution.width() / BASE_CELL_WIDTH as f32;
    let grid_height = (BASE_CELL_WIDTH * 9) / 16;
    let grid = Grid::new(BASE_CELL_WIDTH, grid_height, cell_size);

    let instances: Vec<CellInstance> = (0..grid.cells.len())
        .map(|i| {
            let x = (i % grid.width) as f32 * cell_size - (grid.width as f32 * cell_size) / 2.0
                + cell_size / 2.0;
            let y = (i / grid.width) as f32 * cell_size - (grid.height as f32 * cell_size) / 2.0
                + cell_size / 2.0;
            CellInstance {
                position: Vec2::new(x, y),
                cell_size,
                state: grid.cells[i].state,
            }
        })
        .collect();

    // Single quad mesh (unit square, will be scaled per-instance in shader)
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));

    commands.spawn((
        Mesh2d(mesh),
        CellInstanceData(instances),
        CellGrid,
        GradientIndex(0),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
    ));

    commands.insert_resource(grid);
}

pub fn update_instance_data(
    grid: Option<Res<Grid>>,
    mut query: Query<&mut CellInstanceData, With<CellGrid>>,
    mut gradient_query: Query<&mut GradientIndex, With<CellGrid>>,
) {
    let Some(grid) = grid else { return };
    let Ok(mut instance_data) = query.single_mut() else {
        return;
    };
    for (i, inst) in instance_data.0.iter_mut().enumerate() {
        inst.state = grid.cells[i].state;
    }
    if let Ok(mut gradient_idx) = gradient_query.single_mut() {
        gradient_idx.0 = ColorGradient::all()
            .iter()
            .position(|g| g.name == grid.grid_coloration.gradient.name)
            .unwrap_or(0);
    }
}

pub fn rebuild_grid_instances(
    mut grid: ResMut<Grid>,
    mut query: Query<(&mut CellInstanceData, &mut GradientIndex), With<CellGrid>>,
    windows: Query<&Window>,
) {
    if !grid.needs_rebuild() {
        return;
    }
    let Ok((mut instance_data, mut gradient_idx)) = query.single_mut() else {
        return;
    };
    let window = windows.single().unwrap();
    let new_cell_size = grid.cell_size;
    let width = (window.resolution.width() / new_cell_size) as usize;
    let height = (window.resolution.height() / new_cell_size) as usize;
    let width = width.max(1);
    let height = height.max(1);

    grid.cells = vec![Cell::default(); width * height];
    grid.width = width;
    grid.height = height;
    grid.cell_size = new_cell_size;
    grid.prev_cell_size = new_cell_size;
    grid.init();

    let instances: Vec<CellInstance> = (0..grid.cells.len())
        .map(|i| {
            let x = (i % grid.width) as f32 * new_cell_size
                - (grid.width as f32 * new_cell_size) / 2.0
                + new_cell_size / 2.0;
            let y = (i / grid.width) as f32 * new_cell_size
                - (grid.height as f32 * new_cell_size) / 2.0
                + new_cell_size / 2.0;
            CellInstance {
                position: Vec2::new(x, y),
                cell_size: new_cell_size,
                state: grid.cells[i].state,
            }
        })
        .collect();

    instance_data.0 = instances;
    gradient_idx.0 = 0;
}

// --- GPU buffer ---
#[derive(Component)]
pub struct CellInstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_cell_buffers(
    mut commands: Commands,
    query: Query<(Entity, &CellInstanceData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, data) in &query {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("cell instance buffer"),
            contents: bytemuck::cast_slice(&data.0),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(CellInstanceBuffer {
            buffer,
            length: data.0.len(),
        });
    }
}

// --- Pipeline ---
#[derive(Resource)]
struct CellPipeline {
    shaders: Vec<Handle<Shader>>,
    mesh2d_pipeline: Mesh2dPipeline,
}

fn init_cell_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mesh2d_pipeline: Res<Mesh2dPipeline>,
) {
    let shaders = ColorGradient::all()
        .iter()
        .map(|g| asset_server.load(format!("shaders/{}.wgsl", g.name.to_lowercase())))
        .collect();
    commands.insert_resource(CellPipeline {
        shaders,
        mesh2d_pipeline: mesh2d_pipeline.clone(),
    });
}

impl SpecializedMeshPipeline for CellPipeline {
    type Key = (Mesh2dPipelineKey, usize);

    fn specialize(
        &self,
        (key, gradient_idx): Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut desc = self.mesh2d_pipeline.specialize(key, layout)?;
        let shader = self.shaders[gradient_idx].clone();
        desc.vertex.shader = shader.clone();
        desc.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<CellInstance>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: 12,
                    shader_location: 4,
                },
            ],
        });
        desc.fragment.as_mut().unwrap().shader = shader;
        Ok(desc)
    }
}

// --- Queue ---
fn queue_cells(
    draw_functions: Res<DrawFunctions<Transparent2d>>,
    pipeline: Res<CellPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<CellPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<bevy::render::mesh::RenderMesh>>,
    render_mesh_instances: Res<RenderMesh2dInstances>,
    material_meshes: Query<(Entity, &MainEntity), With<CellInstanceData>>,
    mut transparent_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    views: Query<(&ExtractedView, &Msaa)>,
    gradient_query: Query<&GradientIndex, With<CellInstanceData>>,
) {
    let draw_fn = draw_functions.read().id::<DrawCells>();
    let gradient_idx = gradient_query.iter().next().map(|g| **g).unwrap_or(0);
    for (view, msaa) in &views {
        let Some(phase) = transparent_phases.get_mut(&view.retained_view_entity) else {
            continue;
        };
        let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());
        let view_key = msaa_key | Mesh2dPipelineKey::from_hdr(view.hdr);
        for (entity, main_entity) in &material_meshes {
            let Some(mesh_instance) = render_mesh_instances.get(main_entity) else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let key =
                view_key | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology());
            let pipeline_id = pipelines
                .specialize(&pipeline_cache, &pipeline, (key, gradient_idx), &mesh.layout)
                .unwrap();
            phase.add(Transparent2d {
                entity: (entity, *main_entity),
                draw_function: draw_fn,
                pipeline: pipeline_id,
                sort_key: FloatOrd(0.0),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
                extracted_index: 0,
                indexed: true,
            });
        }
    }
}

// --- Draw command ---
type DrawCells = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    DrawCellsInstanced,
);

struct DrawCellsInstanced;
impl<P: PhaseItem> RenderCommand<P> for DrawCellsInstanced {
    type Param = (
        SRes<RenderAssets<bevy::render::mesh::RenderMesh>>,
        SRes<RenderMesh2dInstances>,
        SRes<bevy::render::mesh::allocator::MeshAllocator>,
    );
    type ViewQuery = ();
    type ItemQuery = Read<CellInstanceBuffer>;

    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: Option<&'w CellInstanceBuffer>,
        (meshes, mesh_instances, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_allocator = mesh_allocator.into_inner();
        let Some(mesh_instance) = mesh_instances.get(&item.main_entity()) else {
            return RenderCommandResult::Skip;
        };
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
            return RenderCommandResult::Skip;
        };
        let Some(inst_buf) = instance_buffer else {
            return RenderCommandResult::Skip;
        };
        let Some(vertex_slice) = mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id)
        else {
            return RenderCommandResult::Skip;
        };

        pass.set_vertex_buffer(0, vertex_slice.buffer.slice(..));
        pass.set_vertex_buffer(1, inst_buf.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            bevy::render::mesh::RenderMeshBufferInfo::Indexed {
                index_format,
                count,
            } => {
                let Some(idx_slice) = mesh_allocator.mesh_index_slice(&mesh_instance.mesh_asset_id)
                else {
                    return RenderCommandResult::Skip;
                };
                pass.set_index_buffer(idx_slice.buffer.slice(..), 0, *index_format);
                pass.draw_indexed(
                    idx_slice.range.start..(idx_slice.range.start + count),
                    vertex_slice.range.start as i32,
                    0..inst_buf.length as u32,
                );
            }
            bevy::render::mesh::RenderMeshBufferInfo::NonIndexed => {
                pass.draw(vertex_slice.range, 0..inst_buf.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
