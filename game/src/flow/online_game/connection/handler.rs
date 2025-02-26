use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{block_on, futures_lite::future::poll_once, AsyncComputeTaskPool, Task},
};

use tungstenite::{connect, stream::MaybeTlsStream};

use crate::states::AppState;

use super::websocket_client::WebSocketClient;

pub struct HandlerPlugin;

impl Plugin for HandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::OnlineGame), setup_connection)
            .add_systems(Update, handle_tasks.run_if(in_state(AppState::OnlineGame)))
            .add_systems(OnExit(AppState::OnlineGame), teardown_connection);
    }
}

#[derive(Component)]
struct WebSocketConnectionSetupTask(Task<Result<CommandQueue, String>>);

fn setup_connection(mut commands: Commands) {
    let url = "ws://127.0.0.1:8000/ws/game";
    let entity = commands.spawn_empty().id();
    {
        let pool = AsyncComputeTaskPool::get();
        let task = pool.spawn(async move {
            let Ok(mut client) = connect(url) else {
                return Err("Failed to connect to server".to_string());
            };
            match client.0.get_mut() {
                MaybeTlsStream::Plain(p) => p.set_nonblocking(true).unwrap(),
                _ => panic!("Unsupported stream type"),
            };
            let mut command_queue = CommandQueue::default();
            command_queue.push(move |world: &mut World| {
                world
                    .entity_mut(entity)
                    .insert(WebSocketClient::new(client.0))
                    .remove::<WebSocketConnectionSetupTask>();
            });

            Ok(command_queue)
        });
        commands
            .entity(entity)
            .insert(WebSocketConnectionSetupTask(task));
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<&mut WebSocketConnectionSetupTask>,
) {
    for mut task in &mut transform_tasks {
        if let Some(result) = block_on(poll_once(&mut task.0)) {
            match result {
                Ok(mut commands_queue) => {
                    commands.append(&mut commands_queue);
                }
                Err(e) => {
                    info!("Connection failed with: {e:?}");
                }
            }
        }
    }
}

fn teardown_connection(
    mut commands: Commands,
    web_socket_clients: Query<Entity, With<WebSocketClient>>,
) {
    for entity in &web_socket_clients {
        commands.entity(entity).remove::<WebSocketClient>();
    }
}
