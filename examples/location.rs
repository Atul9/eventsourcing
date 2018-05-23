#![feature(attr_literals)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate eventsourcing;
extern crate serde_json;
#[macro_use]
extern crate eventsourcing_derive;

use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};

#[derive(Debug)]
struct LocationData {
    lat: f32,
    long: f32,
    alt: f32,
    generation: u64,
}

impl AggregateState for LocationData {
    fn generation(&self) -> u64 {
        self.generation
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("events://github.com/pholactery/eventsourcing/samples/location")]
enum LocationEvent {
    LocationUpdated { lat: f32, long: f32, alt: f32 },
}

impl From<LocationCommand> for LocationEvent {
    fn from(source: LocationCommand) -> Self {
        match source {
            LocationCommand::UpdateLocation { lat, long, alt } => {
                LocationEvent::LocationUpdated { lat, long, alt }
            }
        }
    }
}

struct Location;

enum LocationCommand {
    UpdateLocation { lat: f32, long: f32, alt: f32 },
}

impl Aggregate for Location {
    type Event = LocationEvent;
    type Command = LocationCommand;
    type State = LocationData;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let ld = match evt {
            LocationEvent::LocationUpdated { lat, long, alt } => LocationData {
                lat,
                long,
                alt,
                generation: state.generation + 1,
            },
        };
        Ok(ld)
    }

    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        // SHOULD DO: validate state and command

        // if validation passes...
        Ok(vec![cmd.into()])
    }
}

fn main() {
    let location_store = MemoryEventStore::new();

    let update = LocationCommand::UpdateLocation {
        lat: 10.0,
        long: 52.0,
        alt: 31.0,
    };
    let old_state = LocationData {
        lat: 57.06,
        long: 36.07,
        alt: 15.0,
        generation: 0,
    };

    let evt = LocationEvent::LocationUpdated {
        lat: 30.0,
        long: 20.0,
        alt: 35.0,
    };
    let store_result = location_store.append(evt.clone(), "locations");
    let state = Location::apply_event(&old_state, evt).unwrap();
    let res = Location::handle_command(&old_state, update).unwrap();

    println!("handle command - {:#?}", res);
    println!("append to store - {:#?}", store_result.unwrap());
    println!("apply event - {:#?}", state);
    println!(
        "all events - {:#?}",
        location_store.get_all("locationevent.locationupdated")
    );
}
