use std::sync::Arc;

use k8s_openapi::api::core::v1::Pod;

/// A Kubernetes controller for managing ephemeral pods.
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

    /// Creates the given [`Pod`](k8s_openapi::api::core::v1::Pod).
    ///
    /// Returns when the pod is ready, or an error occured.
    pub async fn spawn(self: Arc<Self>, _pod: &Pod) -> anyhow::Result<EphemeralPod> {
        todo!()
    }
}

/// Reference to a [`Pod`](k8s_openapi::api::core::v1::Pod) spawned with an [`EphemeralPodController`].
///
/// After dropping this reference, the controller should eventually delete the pod.
pub struct EphemeralPod {}

impl AsRef<Pod> for EphemeralPodController {
    fn as_ref(&self) -> &Pod {
        todo!()
    }
}
