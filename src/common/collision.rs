use crate::prelude::*;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

#[derive(Component, Default)]
pub struct Collision {
    pub shape: CollisionShape,
    pub flags: u32,
}

pub struct CollisionQueryEntry {
    pub entity: Entity,
    pub position: Vec2,
    pub shape: CollisionShape,
    pub flags: u32,
}

#[derive(Default)]
pub struct CollisionQuery {
    entries: Vec<CollisionQueryEntry>,
}

#[derive(Copy, Clone)]
pub struct CollisionFilter {
    pub exclude_entity: Entity,
    pub flags: u32,
}

#[derive(Copy, Clone)]
pub struct CollisionQueryResponse {
    pub entity: Entity,
    pub position: Vec2,
    pub shape: CollisionShape,
    pub collide_time: f32,
}

impl CollisionQuery {
    pub fn check(
        &self,
        position: Vec2,
        shape: CollisionShape,
        filter: Option<CollisionFilter>,
    ) -> Option<CollisionQueryResponse> {
        for entry in self.entries.iter() {
            if shape.overlaps(position, entry.shape, entry.position) {
                if let Some(ref filter) = filter {
                    if filter.exclude_entity != entry.entity && filter.flags & entry.flags != 0 {
                        return Some(CollisionQueryResponse {
                            entity: entry.entity,
                            position: entry.position,
                            shape: entry.shape,
                            collide_time: 0.,
                        });
                    }
                } else {
                    return Some(CollisionQueryResponse {
                        entity: entry.entity,
                        position: entry.position,
                        shape: entry.shape,
                        collide_time: 0.,
                    });
                }
            }
        }
        None
    }

    pub fn check_moving(
        &self,
        position: Vec2,
        velocity: Vec2,
        shape: CollisionShape,
        filter: Option<CollisionFilter>,
    ) -> Option<CollisionQueryResponse> {
        let mut result: Option<CollisionQueryResponse> = None;
        for entry in self.entries.iter() {
            if let Some(collide_time) =
                shape.overlaps_moving(position, velocity, entry.shape, entry.position, Vec2::ZERO)
            {
                if let Some(ref filter) = filter {
                    if filter.exclude_entity != entry.entity && filter.flags & entry.flags != 0 {
                        if let Some(other) = result {
                            if collide_time < other.collide_time {
                                result = Some(CollisionQueryResponse {
                                    entity: entry.entity,
                                    position: entry.position,
                                    shape: entry.shape,
                                    collide_time,
                                });
                            }
                        } else {
                            result = Some(CollisionQueryResponse {
                                entity: entry.entity,
                                position: entry.position,
                                shape: entry.shape,
                                collide_time,
                            });
                        }
                    }
                } else {
                    if let Some(other) = result {
                        if collide_time < other.collide_time {
                            result = Some(CollisionQueryResponse {
                                entity: entry.entity,
                                position: entry.position,
                                shape: entry.shape,
                                collide_time,
                            });
                        }
                    } else {
                        result = Some(CollisionQueryResponse {
                            entity: entry.entity,
                            position: entry.position,
                            shape: entry.shape,
                            collide_time,
                        });
                    }
                }
            }
        }
        result
    }

    pub fn check_all(
        &self,
        position: Vec2,
        shape: CollisionShape,
        filter: Option<CollisionFilter>,
    ) -> Vec<Entity> {
        let mut vec: Vec<Entity> = vec![];
        for entry in self.entries.iter() {
            if shape.overlaps(position, entry.shape, entry.position) {
                if let Some(ref filter) = filter {
                    if filter.exclude_entity != entry.entity && filter.flags & entry.flags != 0 {
                        vec.push(entry.entity);
                    }
                } else {
                    vec.push(entry.entity);
                }
            }
        }
        vec
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionQuery>().add_system_to_stage(
            CoreStage::PostUpdate,
            update_collision_query.before(TransformSystem::TransformPropagate),
        );
    }
}

fn update_collision_query(
    query: Query<(Entity, &GlobalTransform, &Collision)>,
    mut collision_query: ResMut<CollisionQuery>,
) {
    collision_query.entries.clear();
    for (entity, transform, collision) in query.iter() {
        collision_query.entries.push(CollisionQueryEntry {
            entity,
            position: transform.translation.truncate(),
            shape: collision.shape,
            flags: collision.flags,
        });
    }
}
