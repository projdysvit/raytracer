var<private> VERTICES: array<vec2<f32>, 3> = array(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(-1.0, 3.0),
    vec2<f32>(3.0, -1.0)
);

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {
    return VertexOutput(
        vec4<f32>(VERTICES[i], 0.0, 1.0),
        (VERTICES[i] + 1.0) * 0.5
    );
}

struct CameraUniform {
    position: vec3<f32>,
    aspect_ratio: f32
}

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
    reflectivity: f32,
    shininess: f32
}

struct Light {
    position: vec3<f32>,
    intensity: f32,
    color: vec3<f32>
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>
}

struct Intersection {
    distance: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
    color: vec3<f32>,
    reflectivity: f32,
    shininess: f32
}

const MAX_BOUNCES: i32 = 9;
const EPSILON: f32 = 0.001;

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(2) var<uniform> light: Light;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray_position = camera.position;
    let ray_direction = vec3<f32>(
        (in.uv.x - 0.5) * camera.aspect_ratio,
        (0.5 - in.uv.y),
        -1.0
    );

    var ray = Ray(ray_position, ray_direction);
    var intersection: Intersection;
    if intersect_spheres(&intersection, ray) {
        var color = shade(intersection, -ray.direction);
        var reflection_strength = vec3<f32>(intersection.reflectivity);

        for (var bounce = 0; bounce < MAX_BOUNCES; bounce++) {
            if (reflection_strength.x + reflection_strength.y + reflection_strength.z < EPSILON) {
                break;
            }

            let bounce_ray = Ray(intersection.position, reflect(ray.direction, intersection.normal));

            var bounce_intersection: Intersection;
            if intersect_spheres(&bounce_intersection, bounce_ray) {
                color += shade(bounce_intersection, -bounce_ray.direction) * reflection_strength;

                intersection = bounce_intersection;
                ray = bounce_ray;
                reflection_strength *= intersection.reflectivity;
            } else {
                color += sky_color(bounce_ray) * reflection_strength;

                break;
            }
        }

        return vec4<f32>(color, 1.0);
    }

    return vec4<f32>(sky_color(ray), 1.0);
}

fn intersect_spheres(intersection: ptr<function, Intersection>, ray: Ray) -> bool {
    var has_intersection = false;
    var nearest_intersection_distance = 1e30;

    for (var i = 0u; i < arrayLength(&spheres); i++) {
        let sphere_to_ray_position = ray.origin - spheres[i].center;
        let ray_direction_dot = dot(ray.direction, ray.direction);
        let ray_position_dot = 2.0 * dot(sphere_to_ray_position, ray.direction);
        let sphere_radius_squared = dot(sphere_to_ray_position, sphere_to_ray_position) - spheres[i].radius * spheres[i].radius;
        let discriminant = ray_position_dot * ray_position_dot - 4.0 * ray_direction_dot * sphere_radius_squared;

        if discriminant > 0.0 {
            let t1 = (-ray_position_dot - sqrt(discriminant)) / (2.0 * ray_direction_dot);
            let t2 = (-ray_position_dot + sqrt(discriminant)) / (2.0 * ray_direction_dot);
            let intersection_distance = min(t1, t2);
            if intersection_distance > EPSILON && intersection_distance < nearest_intersection_distance {
                nearest_intersection_distance = intersection_distance;
                has_intersection = true;
                (*intersection).distance = intersection_distance;
                (*intersection).position = ray.origin + intersection_distance * ray.direction;
                (*intersection).normal = normalize((*intersection).position - spheres[i].center);
                (*intersection).color = spheres[i].color;
                (*intersection).reflectivity = spheres[i].reflectivity;
                (*intersection).shininess = spheres[i].shininess;
            }
        }
    }

    return has_intersection;
}

fn shade(intersection: Intersection, view: vec3<f32>) -> vec3<f32> {
    var result = vec3<f32>(0.0);

    let light_direction = normalize(light.position - intersection.position);
    let shadow_ray = Ray(intersection.position + intersection.normal * EPSILON, light_direction);

    if !in_shadow(shadow_ray) {
        let n_dot_l = max(dot(intersection.normal, light_direction), 0.0);
        let diffuse = intersection.color * n_dot_l * light.intensity * light.color;
        let half_vector = normalize(light_direction + view);
        let n_dot_h = max(dot(intersection.normal, half_vector), 0.0);
        let specular = light.color * pow(n_dot_h, intersection.shininess) * light.intensity;

        result = diffuse + specular;
    }

    return result;
}

fn in_shadow(shadow_ray: Ray) -> bool {
    var shadow_intersection: Intersection;
    return intersect_spheres(&shadow_intersection, shadow_ray);
}

fn sky_color(ray: Ray) -> vec3<f32> {
    let gradient = 0.5 * (normalize(ray.direction).y + 1.0);
    let top_screen = gradient * vec3<f32>(0.3, 0.5, 1.0);
    let bottom_screen = (1.0 - gradient) * vec3<f32>(1.0);

    return top_screen + bottom_screen;
}
