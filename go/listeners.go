package main

import "net"

// EventListeners allow for listening for events coming inside UDP packets.
//
// A running application should use only one instance.
type EventListeners interface {
	// Listen returns an Events instance that yields only events matching the given `eventFilter`.
	//
	// This method can be used concurrently by multiple callers.
	// Each event captured on the given `address` will be broadcasted
	// to all Events instances with matching `eventFilter`s.
	//
	// For the `eventFilter` to match an Event,
	// Event.Payload must be a superset of the `eventFilter.
	Listen(address net.UDPAddr, eventFilter map[string]string) (Events, error)
}

// Event decoded from a UDP packet.
type Event struct {
	// Source address of the original packet.
	SourceAddress net.UDPAddr
	// Body of the original packet, deserialized from JSON.
	Payload map[string]string
}

type Events interface {
	Next() (Event, error)
}
