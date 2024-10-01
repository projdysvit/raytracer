struct VertexInput {
    @location(0) position: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    return VertexOutput(
        vec4<f32>(in.position, 0.0, 1.0),
        (in.position + 1.0) * 0.5
    );
}

struct CameraUniform {
    aspect_ratio: f32,
    width: f32,
    height: f32
}

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
    reflectivity: f32
}

struct Light {
    position: vec3<f32>,
    intensity: f32
}

struct Ray {
    position: vec3<f32>,
    direction: vec3<f32>
}

struct Intersection {
    t: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
    color: vec3<f32>,
    reflectivity: f32
}

const MAX_BOUNCES: i32 = 9;
const EPSILON: f32 = 0.001;

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(2) var<storage, read> light: Light;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray_position = vec3<f32>(0.0, 0.0, 0.0);
    let ray_direction = vec3<f32>(
        (in.uv.x - 0.5) * camera.aspect_ratio,
        (0.5 - in.uv.y),
        -1.0
    );

    var ray = Ray(ray_position, ray_direction);
    var intersection: Intersection;
    if intersect_spheres(&intersection, ray) {
        var color = shade(intersection.color, intersection.position, intersection.normal, -ray.direction);
        var reflection_strength = vec3<f32>(intersection.reflectivity);

        for (var bounce = 0; bounce < MAX_BOUNCES; bounce++) {
            if (reflection_strength.x + reflection_strength.y + reflection_strength.z < EPSILON) {
                break;
            }

            let bounce_ray = Ray(intersection.position, reflect(ray.direction, intersection.normal));

            var bounce_intersection: Intersection;
            if intersect_spheres(&bounce_intersection, bounce_ray) {
                color += shade(
                    bounce_intersection.color,
                    bounce_intersection.position,
                    bounce_intersection.normal,
                    -bounce_ray.direction
                ) * reflection_strength;

                intersection = bounce_intersection;
                ray = bounce_ray;
                reflection_strength *= intersection.reflectivity;
            } else {
                break;
            }
        }

        return vec4<f32>(color, 1.0);
    }

    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

fn intersect_spheres(intersection: ptr<function, Intersection>, ray: Ray) -> bool {
    var hit = false;
    var closest_t = 1e30;

    for (var i = 0u; i < arrayLength(&spheres); i++) {
        let sphere = spheres[i];
        let oc = ray.position - sphere.center;
        let a = dot(ray.direction, ray.direction);
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t1 = (-b - sqrt(discriminant)) / (2.0 * a);
            let t2 = (-b + sqrt(discriminant)) / (2.0 * a);
            let t = min(t1, t2);
            if t > EPSILON && t < closest_t {
                closest_t = t;
                hit = true;
                (*intersection).t = t;
                (*intersection).position = ray.position + t * ray.direction;
                (*intersection).normal = normalize((*intersection).position - sphere.center);
                (*intersection).color = spheres[i].color;
                (*intersection).reflectivity = spheres[i].reflectivity;
            }
        }
    }

    return hit;
}

fn shade(color: vec3<f32>, position: vec3<f32>, normal: vec3<f32>, view: vec3<f32>) -> vec3<f32> {
    var result = vec3<f32>(0.0);

    let to_light = normalize(light.position - position);
    let shadow_ray = Ray(position + normal * EPSILON, to_light);

    if !in_shadow(shadow_ray) {
        let light_direction = normalize(light.position - position);
        let n_dot_l = max(dot(normal, light_direction), 0.0);
        result += color * n_dot_l * vec3<f32>(light.intensity);
    }

    return result;
}

fn in_shadow(shadow_ray: Ray) -> bool {
    var shadow_intersection: Intersection;
    return intersect_spheres(&shadow_intersection, shadow_ray);
}
