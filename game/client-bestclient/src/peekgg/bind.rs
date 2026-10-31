macro_rules! define_binds {
    (
        bool { $($bv:ident => $($bp:ident).+),* $(,)? }
        int { $($iv:ident => $($ip:ident).+),* $(,)? }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Bind {
            $($bv,)*
            $($iv,)*
        }

        impl Bind {
            pub fn by_utf8xbot_4101(self, cfg: &Config) -> i64 {
                match self {
                    $(Bind::$bv => cfg.$($bp).+ as i64,)*
                    $(Bind::$iv => cfg.$($ip).+ as i64,)*
                }
            }

            pub fn by_utf8xbot_4102(self, cfg: &mut Config, val: i64) {
                match self {
                    $(Bind::$bv => cfg.$($bp).+ = val != 0,)*
                    $(Bind::$iv => cfg.$($ip).+ = val as _,)*
                }
            }
        }
    };
}

use game_config::config::Config;

define_binds! {
    bool {
        ShowFps => game.cl.show_fps,
        Nameplates => game.cl.render.nameplates,
        OwnNameplate => game.cl.render.own_nameplate,
    }
    int {
        Opacity => game.peekgg.cl.opacity,
        Scale => game.peekgg.cl.scale,
        Aura => game.peekgg.cl.aura,
        AuraRadius => game.peekgg.cl.aura_radius,
        AuraAlpha => game.peekgg.cl.aura_alpha,
        AuraRainbow => game.peekgg.cl.aura_rainbow,
        Outline => game.peekgg.cl.outline,
        OutlineRainbow => game.peekgg.cl.outline_rainbow,
        JumpRing => game.peekgg.cl.jump_ring,
        RingRainbow => game.peekgg.cl.ring_rainbow,
        Accessory => game.peekgg.cl.accessory,
        HatRainbow => game.peekgg.cl.hat_rainbow,
        BcShowRealHitbox => game.peekgg.bc.show_real_hitbox,
        BcShowCorrectCheckpoint => game.peekgg.bc.show_correct_checkpoint,
        BcShowPointsInTab => game.peekgg.bc.show_points_in_tab,
        BcShowSpectatorNames => game.peekgg.bc.show_spectator_names,
        BcShowVotePercentage => game.peekgg.bc.show_vote_percentage,
        BcScoreboardScale => game.peekgg.bc.scoreboard_scale,
        BcNameplateGradient => game.peekgg.bc.nameplate_gradient,
        BcNameplateGradientMode => game.peekgg.bc.nameplate_gradient_mode,
        BcNameplateGradientClan => game.peekgg.bc.nameplate_gradient_clan,
        BcNameplateGradientEverything => game.peekgg.bc.nameplate_gradient_everything,
        BcNameplateGradientAnimateSpeed => game.peekgg.bc.nameplate_gradient_animate_speed,
        BcFreezePull => game.peekgg.bc.freeze_pull,
        BcFreezePullShield => game.peekgg.bc.freeze_pull_shield,
        BcFreezePullRange => game.peekgg.bc.freeze_pull_range,
        BcFreezePullStrands => game.peekgg.bc.freeze_pull_strands,
        BcFreezePullSparks => game.peekgg.bc.freeze_pull_sparks,
        BcFreezePullHookColor => game.peekgg.bc.freeze_pull_hook_color,
        BcHookStyle => game.peekgg.bc.hook_style,
        BcHookTip => game.peekgg.bc.hook_tip,
        BcHookChargeTime => game.peekgg.bc.hook_charge_time,
        BcHookSize => game.peekgg.bc.hook_size,
        BcLaserGlow => game.peekgg.bc.laser_glow,
        BcLaserGlowPower => game.peekgg.bc.laser_glow_power,
        BcShotgunGlow => game.peekgg.bc.shotgun_glow,
        BcShotgunGlowPower => game.peekgg.bc.shotgun_glow_power,
        BcJellyTee => game.peekgg.bc.jelly_tee,
        BcJellyTeeOthers => game.peekgg.bc.jelly_tee_others,
        BcJellyTeeStrength => game.peekgg.bc.jelly_tee_strength,
        BcGoresMode => game.peekgg.bc.gores_mode,
        BcParticles3d => game.peekgg.bc.particles_3d,
        BcParticles3dDensity => game.peekgg.bc.particles_3d_density,
        BcParticles3dSizeMax => game.peekgg.bc.particles_3d_size_max,
        BcParticles3dGlow => game.peekgg.bc.particles_3d_glow,
        BcCinematicCamera => game.peekgg.bc.cinematic_camera,
        BcCinematicCameraStrength => game.peekgg.bc.cinematic_camera_strength,
        BcDynamicFov => game.peekgg.bc.dynamic_fov,
        BcBetterSpectate => game.peekgg.bc.better_spectate,
        BcSpecMovedNotify => game.peekgg.bc.spec_moved_notify,
        BcCursorTrail => game.peekgg.bc.cursor_trail,
        BcCursorTrailTrailSize => game.peekgg.bc.cursor_trail_size,
        BcCursorTrailOpacity => game.peekgg.bc.cursor_trail_opacity,
        BcAtmosphere => game.peekgg.bc.atmosphere,
        BcWeather => game.peekgg.bc.weather,
        BcWeatherAmount => game.peekgg.bc.weather_amount,
        BcWeatherRainSize => game.peekgg.bc.weather_rain_size,
        BcWeatherRainSpeed => game.peekgg.bc.weather_rain_speed,
        BcCustomAspectRatioMode => game.peekgg.bc.custom_aspect_ratio_mode,
        BcCustomAspectRatio => game.peekgg.bc.custom_aspect_ratio,
        BcCustomAspectRatioNum => game.peekgg.bc.custom_aspect_ratio_num,
        BcCustomAspectRatioDen => game.peekgg.bc.custom_aspect_ratio_den,
        BcFrozenSkinDarken => game.peekgg.bc.frozen_skin_darken,
        BcShowhudDummyCoordIndicator => game.peekgg.bc.showhud_dummy_coord_indicator,
        BcAnimations => game.peekgg.bc.animations,
        BcConfirmQuit => game.peekgg.bc.confirm_quit,
        BcWheelScale => game.peekgg.bc.wheel_scale,
        BcMuteOthersHook => game.peekgg.bc.mute_others_hook,
        BcMuteOthersHammer => game.peekgg.bc.mute_others_hammer,
        BcMuteOthersAirJump => game.peekgg.bc.mute_others_air_jump,
        ClShowHud => game.peekgg.cl.show_hud,
        ClShowHudHealthAmmo => game.peekgg.cl.show_hud_health_ammo,
        ClShowHudDdrace => game.peekgg.cl.show_hud_ddrace,
        ClShowHudJumpsIndicator => game.peekgg.cl.show_hud_jumps_indicator,
        ClShowHudTimer => game.peekgg.cl.show_hud_timer,
        ClShowHudTimeCpDiff => game.peekgg.cl.show_hud_time_cp_diff,
        ClShowHudPlayerPosition => game.peekgg.cl.show_hud_player_position,
        ClShowHudPlayerSpeed => game.peekgg.cl.show_hud_player_speed,
        ClShowHudSpectatorCount => game.peekgg.cl.show_hud_spectator_count,
        ClScoreboardOnDeath => game.peekgg.cl.scoreboard_on_death,
        ClShowIds => game.peekgg.cl.show_ids,
        ClShowOthers => game.peekgg.cl.show_others,
        ClShowOthersAlpha => game.peekgg.cl.show_others_alpha,
        ClOverlayEntities => game.peekgg.cl.overlay_entities,
        ClShowFreezeBars => game.peekgg.cl.show_freeze_bars,
        ClShowHookCollOwn => game.peekgg.cl.show_hook_coll_own,
        ClShowHookCollOther => game.peekgg.cl.show_hook_coll_other,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorBind {
    Aura,
    Outline,
    Ring,
    Hat,
    FreezePull,
}

impl ColorBind {
    pub fn by_utf8xbot_4103(self, cfg: &Config) -> egui::Color32 {
        let rgb = match self {
            ColorBind::Aura => cfg.game.peekgg.cl.aura_color,
            ColorBind::Outline => cfg.game.peekgg.cl.outline_color,
            ColorBind::Ring => cfg.game.peekgg.cl.ring_color,
            ColorBind::Hat => cfg.game.peekgg.cl.hat_color,
            ColorBind::FreezePull => cfg.game.peekgg.bc.freeze_pull_color,
        };
        egui::Color32::from_rgb(rgb.r, rgb.g, rgb.b)
    }

    pub fn by_utf8xbot_4104(self, cfg: &mut Config, color: egui::Color32) {
        let mut rgb = match self {
            ColorBind::Aura => cfg.game.peekgg.cl.aura_color,
            ColorBind::Outline => cfg.game.peekgg.cl.outline_color,
            ColorBind::Ring => cfg.game.peekgg.cl.ring_color,
            ColorBind::Hat => cfg.game.peekgg.cl.hat_color,
            ColorBind::FreezePull => cfg.game.peekgg.bc.freeze_pull_color,
        };
        rgb.r = color.r();
        rgb.g = color.g();
        rgb.b = color.b();
        match self {
            ColorBind::Aura => cfg.game.peekgg.cl.aura_color = rgb,
            ColorBind::Outline => cfg.game.peekgg.cl.outline_color = rgb,
            ColorBind::Ring => cfg.game.peekgg.cl.ring_color = rgb,
            ColorBind::Hat => cfg.game.peekgg.cl.hat_color = rgb,
            ColorBind::FreezePull => cfg.game.peekgg.bc.freeze_pull_color = rgb,
        }
    }
}
