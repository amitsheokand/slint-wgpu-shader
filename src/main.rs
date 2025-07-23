use slint_shader::AnimatedShaderManager;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize WGPU backend for Slint
    slint::BackendSelector::new()
        .require_wgpu_24(slint::wgpu_24::WGPUConfiguration::default())
        .select()?;

    let app = WgpuShaderApp::new()?;
    let app_weak = app.as_weak();

    // Create a shared shader manager
    let shader_manager: Rc<RefCell<Option<AnimatedShaderManager>>> = Rc::new(RefCell::new(None));
    let shader_manager_clone = shader_manager.clone();

    // Set up rendering notifier to create shader textures
    app.window()
        .set_rendering_notifier(move |state, graphics_api| {
            let (
                Some(app),
                slint::RenderingState::RenderingSetup,
                slint::GraphicsAPI::WGPU24 { device, queue, .. },
            ) = (app_weak.upgrade(), state, graphics_api)
            else {
                return;
            };

            // Initialize the shader manager
            let manager = pollster::block_on(AnimatedShaderManager::new(&device, &queue));
            *shader_manager_clone.borrow_mut() = Some(manager);

            // Initial render
            if let Some(ref mut manager) = shader_manager_clone.borrow_mut().as_mut() {
                let (rainbow_image, wave_image, noise_image, gradient_image) =
                    manager.update_and_render(app.get_animation_enabled());

                app.set_rainbow_shader_texture(rainbow_image);
                app.set_wave_shader_texture(wave_image);
                app.set_noise_shader_texture(noise_image);
                app.set_gradient_shader_texture(gradient_image);
            }
        })?;

    // Set up a timer for continuous animation
    let timer = slint::Timer::default();
    let app_weak_timer = app.as_weak();
    let shader_manager_timer = shader_manager.clone();

    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(16), // ~60 FPS
        move || {
            if let (Some(app), Some(ref mut manager)) = (
                app_weak_timer.upgrade(),
                shader_manager_timer.borrow_mut().as_mut(),
            ) {
                // Always render, but pass animation state to control time progression
                let animation_enabled = app.get_animation_enabled();
                let (rainbow_image, wave_image, noise_image, gradient_image) =
                    manager.update_and_render(animation_enabled);

                app.set_rainbow_shader_texture(rainbow_image);
                app.set_wave_shader_texture(wave_image);
                app.set_noise_shader_texture(noise_image);
                app.set_gradient_shader_texture(gradient_image);
            }
        },
    );

    app.run()?;
    Ok(())
}
