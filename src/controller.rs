use std::sync::Arc;

use k8s_openapi::api::core::v1::Pod;

/// A Kubernetes controller for managing ephemeral pods.
///
/// Meant to be run within a service deployed to Kubernetes,
/// and used by internal task to spawn short-lived [`Pod`]s.
pub struct EphemeralPodController {
    // ...todo
}

impl EphemeralPodController {
    /// Runs this controller.
    ///
    /// The controller should shutdown gracefully when the `graceful_shutdown` [`Future`] completes.
    pub async fn run<F>(self: Arc<Self>, _graceful_shutdown: F)
    where
        F: 'static + Send + Sync + Future<Output = ()>,
    {
        todo!()
    }

    /// Spawns the given [`Pod`].
    ///
    /// Returns when the pod is ready, or an error occured.
    pub async fn spawn(self: Arc<Self>, _pod: &Pod) -> anyhow::Result<EphemeralPod> {
        todo!()
    }
}

/// Reference/handle to a [`Pod`] spawned with an [`EphemeralPodController`].
///
/// After dropping this reference, the controller should eventually delete the pod.
pub struct EphemeralPod {}

impl EphemeralPod {
    /// Returns the last observed state of the [`Pod`].
    pub fn get(&self) -> Arc<Pod> {
        todo!()
    }

    /// Returns when the observed state of the [`Pod`] has changed.
    pub async fn changed(&mut self) {
        todo!()
    }
}
