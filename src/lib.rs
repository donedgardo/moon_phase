pub mod moon;
pub mod chat;

#[cfg(test)]
mod ecs_test {
    use bevy_ecs::prelude::*;

    #[derive(Component)]
    struct Position {
        x: f32,
        y: f32,
    }
    #[derive(Component)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    // This system moves each entity with a Position and Velocity component
    fn movement(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in &mut query {
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }

    #[test]
    fn main() {
        // Create a new empty World to hold our Entities and Components
        let mut world = World::new();

        // Spawn an entity with Position and Velocity components
        let entity_id = world
            .spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }))
            .id();

        // Create a new Schedule, which defines an execution strategy for Systems
        let mut schedule = Schedule::default();

        // Add our system to the schedule
        schedule.add_systems(movement);

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);
        let position = world.entity(entity_id).get::<Position>().unwrap();
        assert_eq!(position.x, 1.);
        assert_eq!(position.y, 1.);
    }
}
