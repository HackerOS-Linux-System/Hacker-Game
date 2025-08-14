// assets/shaders/neon.wgsl
struct Time {
    time: f32,
};

@group(0) @binding(0)
var<uniform> time: Time;

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    // Gradient neonowy z pulsowaniem
    let t = position.x / 800.0 + sin(time.time * 2.0) * 0.1;
    let pulse = sin(time.time * 5.0) * 0.2 + 0.8; // Migotanie
    return vec4<f32>(0.0, t * pulse, 1.0 * pulse, 1.0); // Niebieski neon z pulsowaniem
}
