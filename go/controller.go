package main

import (
	"context"
	corev1 "k8s.io/api/core/v1"
)

// EphemeralPodController is a Kubernetes controller for managing ephemeral pods.
type EphemeralPodController interface {
	// Run runs this controller.
	Run(context *context.Context)

	// Spawn creates a new ephemeral Pod.
	//
	// Returns when the pod is ready.
	Spawn(pod *corev1.Pod) (EphemeralPod, error)
}

type EphemeralPod interface {
	// Pod returns a reference to the pod.
	Pod() *corev1.Pod

	// Release marks that the pod is no longer needed,
	// and can be garbage collected by the EphemeralPodController.
	Release()
}
