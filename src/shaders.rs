pub const RAINBOW_SHADER: &str = r#"
    struct TimeUniform {
        time: f32,
    }
    @group(0) @binding(0) var<uniform> time_uniform: TimeUniform;

    @vertex
    fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
        var pos = array<vec2<f32>, 6>(
            vec2<f32>(-1.0, -1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>( 1.0,  1.0)
        );
        return vec4<f32>(pos[vertex_index], 0.0, 1.0);
    }

    @fragment
    fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
        let uv = position.xy / vec2<f32>(320.0, 200.0);
        let time = time_uniform.time;
        
        // Animated rainbow effect
        let color = vec3<f32>(
            sin(uv.x * 10.0 + time * 2.0) * 0.5 + 0.5,
            sin(uv.y * 10.0 + time * 2.0 + 2.094) * 0.5 + 0.5,
            sin((uv.x + uv.y) * 10.0 + time * 2.0 + 4.188) * 0.5 + 0.5
        );
        return vec4<f32>(color, 1.0);
    }
"#;

pub const WAVE_SHADER: &str = r#"
    struct TimeUniform {
        time: f32,
    }
    @group(0) @binding(0) var<uniform> time_uniform: TimeUniform;

    @vertex
    fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
        var pos = array<vec2<f32>, 6>(
            vec2<f32>(-1.0, -1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>( 1.0,  1.0)
        );
        return vec4<f32>(pos[vertex_index], 0.0, 1.0);
    }

    @fragment
    fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
        let uv = position.xy / vec2<f32>(320.0, 200.0);
        let time = time_uniform.time;
        
        // Animated wave effect with proper UV coordinates
        let wave1 = sin(uv.x * 20.0 + time * 3.0) * 0.5 + 0.5;
        let wave2 = sin(uv.y * 15.0 + time * 2.0) * 0.3 + 0.7;
        let combined = wave1 * wave2;
        
        // Add some movement to the waves
        let moving_wave = sin((uv.x + uv.y) * 10.0 + time * 4.0) * 0.2 + 0.8;
        
        let color = vec3<f32>(
            combined * moving_wave,
            combined * 0.5 + sin(time * 1.5) * 0.2,
            combined * 0.8 + cos(time * 1.2) * 0.3
        );
        return vec4<f32>(color, 1.0);
    }
"#;

pub const NOISE_SHADER: &str = r#"
    struct TimeUniform {
        time: f32,
    }
    @group(0) @binding(0) var<uniform> time_uniform: TimeUniform;

    @vertex
    fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
        var pos = array<vec2<f32>, 6>(
            vec2<f32>(-1.0, -1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>( 1.0,  1.0)
        );
        return vec4<f32>(pos[vertex_index], 0.0, 1.0);
    }

    fn hash(p: vec2<f32>) -> f32 {
        var p3 = fract(p.xyx * vec3<f32>(.1031, .1030, .0973));
        p3 += dot(p3, p3.yxz + 33.33);
        return fract((p3.x + p3.y) * p3.z);
    }

    @fragment
    fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
        let uv = position.xy / vec2<f32>(320.0, 200.0);
        let time = time_uniform.time;
        
        // Animated noise effect
        let animated_uv = uv + vec2<f32>(sin(time * 0.5), cos(time * 0.3)) * 0.1;
        let noise = hash(animated_uv * 50.0 + time * 2.0);
        
        // Color cycling noise
        let color = vec3<f32>(
            noise,
            noise * 0.7 + sin(time * 2.0) * 0.3,
            noise * 0.3 + cos(time * 1.5) * 0.4
        );
        return vec4<f32>(color, 1.0);
    }
"#;

pub const GRADIENT_SHADER: &str = r#"
    struct TimeUniform {
        time: f32,
    }
    @group(0) @binding(0) var<uniform> time_uniform: TimeUniform;

    @vertex
    fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
        var pos = array<vec2<f32>, 6>(
            vec2<f32>(-1.0, -1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>(-1.0,  1.0),
            vec2<f32>( 1.0, -1.0),
            vec2<f32>( 1.0,  1.0)
        );
        return vec4<f32>(pos[vertex_index], 0.0, 1.0);
    }

    @fragment
    fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
        let uv = position.xy / vec2<f32>(320.0, 200.0);
        let time = time_uniform.time;
        
        // Animated rotating gradient
        let center = vec2<f32>(0.5, 0.5);
        let offset_uv = uv - center;
        
        // Rotate the gradient over time
        let angle = time * 0.5;
        let cos_a = cos(angle);
        let sin_a = sin(angle);
        let rotated_uv = vec2<f32>(
            offset_uv.x * cos_a - offset_uv.y * sin_a,
            offset_uv.x * sin_a + offset_uv.y * cos_a
        ) + center;
        
        let color = vec3<f32>(
            rotated_uv.x + sin(time) * 0.2,
            rotated_uv.y + cos(time * 1.3) * 0.2,
            (rotated_uv.x + rotated_uv.y) * 0.5 + sin(time * 0.8) * 0.3
        );
        return vec4<f32>(color, 1.0);
    }
"#;
