use bevy::prelude::*;

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
