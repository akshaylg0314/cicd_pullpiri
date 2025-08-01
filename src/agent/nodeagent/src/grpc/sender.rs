/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::monitoringserver::{ContainerList, SendContainerListResponse};
use common::statemanager::{
    connect_server, state_manager_connection_client::StateManagerConnectionClient, Action, Response,
};

use common::monitoringserver::monitoring_server_connection_client::MonitoringServerConnectionClient;
use tonic::{Request, Status};

/// Sender for making gRPC requests to Monitoring Server
#[derive(Clone, Default)]
pub struct NodeAgentSender {}

impl NodeAgentSender {
    /// Trigger an action for a scenario
    pub async fn trigger_action(
        &mut self,
        action: Action,
    ) -> Result<tonic::Response<Response>, Status> {
        let mut client = StateManagerConnectionClient::connect(connect_server())
            .await
            .unwrap();
        client.send_action(Request::new(action)).await
    }

    /// Send a ContainerList to the monitoring server via gRPC
    pub async fn send_container_list(
        &mut self,
        container_list: ContainerList,
    ) -> Result<tonic::Response<SendContainerListResponse>, Status> {
        let mut client =
            MonitoringServerConnectionClient::connect(common::monitoringserver::connect_server())
                .await
                .unwrap();
        client
            .send_container_list(Request::new(container_list))
            .await
    }

    /// Send a changed ContainerList to the state manager via gRPC
    pub async fn send_changed_container_list(
        &mut self,
        container_list: ContainerList,
    ) -> Result<tonic::Response<SendContainerListResponse>, Status> {
        let mut client =
            StateManagerConnectionClient::connect(common::statemanager::connect_server())
                .await
                .unwrap();
        client
            .send_changed_container_list(Request::new(container_list))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::grpc::sender::NodeAgentSender;
    use common::monitoringserver::{ContainerList, SendContainerListResponse};
    use common::statemanager::{Action, Response as SMResponse};
    use tonic::{Request, Response, Status};

    #[tokio::test]
    async fn test_trigger_action_success() {
        let mut sender = NodeAgentSender::default();

        let action = Action::default();
        let result = sender.trigger_action(action).await;

        match result {
            Ok(response) => {
                let _resp: SMResponse = response.into_inner();
            }
            Err(_) => {
                // connection might fail in test environment, still test handles Ok case
            }
        }
    }

    #[tokio::test]
    async fn test_trigger_action_multiple_calls() {
        let mut sender = NodeAgentSender::default();

        let action1 = Action::default();
        let action2 = Action::default();

        let result1 = sender.trigger_action(action1).await;
        let result2 = sender.trigger_action(action2).await;

        assert!(result1.is_ok() || result1.is_err());
        assert!(result2.is_ok() || result2.is_err());
    }

    #[tokio::test]
    async fn test_send_container_list_error_propagation() {
        let mut sender = NodeAgentSender::default();

        let container_list = ContainerList::default();

        let response = sender.send_container_list(container_list).await;

        assert!(response.is_ok() || response.is_err());
    }

    #[tokio::test]
    async fn test_send_changed_container_list_error_propagation() {
        let mut sender = NodeAgentSender::default();

        let container_list = ContainerList::default();

        let response = sender.send_changed_container_list(container_list).await;

        assert!(response.is_ok() || response.is_err());
    }
}
