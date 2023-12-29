use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceObject {
    pub(crate) messageType: i32,
    pub(crate) viewport: Vec2,
    pub(crate) viewportScale: f64,
    pub(crate) sessionId: String,
    pub(crate) acceleration: Vec2,
    pub(crate) ammo: i32,
    pub(crate) angleDegree: f32,
    pub(crate) angularVelocity: f64,
    pub(crate) armedDelay: i32,
    pub(crate) batteryLevel: f32,
    pub(crate) batteryCapacity: i32,
    pub(crate) booster: i32,
    pub(crate) bounceCount: i32,
    pub(crate) canonCoolDown: i32,
    pub(crate) canonCoolDownSpeed: f64,
    pub(crate) canonHeatAddedPerShot: f64,
    pub(crate) canonOverHeat: bool,
    pub(crate) colliding: bool,
    pub(crate) collidingWith: Vec<String>,
    pub(crate) color: String,
    pub(crate) damage: i32,
    pub(crate) deadFrameCount: i32,
    pub(crate) didHit: bool,
    pub(crate) enginePower: f64,
    pub(crate) framesSinceLastServerUpdate: i32,
    pub(crate) framesSinceLastShot: i32,
    pub(crate) health: i32,
    pub(crate) startHealth: i32,
    pub(crate) hitRadius: f32,
    pub(crate) inverseFireRate: i32,
    pub(crate) isDead: bool,
    pub(crate) isLocal: bool,
    pub(crate) isPlaying: bool,
    pub(crate) killedByName: String,
    pub(crate) kills: Vec<String>,
    pub(crate) killCount: i32,
    pub(crate) mass: i32,
    pub(crate) missileDamage: i32,
    pub(crate) missileSpeed: i32,
    pub(crate) motivationLevel: i32,
    pub(crate) motivatorBroken: bool,
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) obliterated: bool,
    pub(crate) online: bool,
    pub(crate) photonColor: String,
    pub(crate) position: Vec2,
    pub(crate) serverVersion: String,
    pub(crate) shape: SpaceShape,
    pub(crate) shotBlowFrame: i32,
    pub(crate) shotsInFlight: Vec<Shot>,
    pub(crate) shotsInFlightNew: Vec<Shot>,
    pub(crate) shotsFiredThisFrame: bool,
    pub(crate) shotsPerFrame: i32,
    pub(crate) size: Vec2,
    pub(crate) steeringPower: f64,
    pub(crate) velocity: Vec2,
    pub(crate) ownerName: String,
    pub(crate) lastDamagedByName: String,
    pub(crate) joinedGame: String, // Assuming `currentTimeDate()` returns a String
    pub(crate) lastMessage: String,
    pub(crate) readyToPlay: bool,
    pub(crate) isHost: bool,
    pub(crate) ping: bool,
    pub(crate) pingResponse: bool,
    pub(crate) pingId: String,
    pub(crate) hops: i32,
    pub(crate) ttl: i32,
    pub(crate) rtt: i32,
    pub(crate) worldSize: Vec2,
    pub(crate) cameraPosition: Vec2,
    pub(crate) cameraVelocity: Vec2,
    pub(crate) viewFramePosition: Vec2,
    pub(crate) thrustFlames: Vec<ThrustFlame>,
    pub(crate) ship: Ship,
    pub(crate) moonType: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vec2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shot {
    // Define fields for the Shot struct as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpaceShape {
    SmallShip, // Add other variants as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThrustFlame {
    // Define fields for the ThrustFlame struct as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    pub(crate) name: String,
    pub(crate) level: i32,
    pub(crate) userId: String,
    pub(crate) shipVariant: i32,
    pub(crate) id: String,
    pub(crate) experience: i32,
}