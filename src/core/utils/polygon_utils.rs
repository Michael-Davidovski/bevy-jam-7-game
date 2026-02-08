use bevy::{asset::RenderAssetUsages, prelude::*, math::*};

pub fn create_polygon_mesh(points: &[Vec2]) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default()
    );

    // Triangulate the polygon (simple fan triangulation for convex polygons)
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for point in points {
        vertices.push([point.x, point.y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.0, 0.0]); // Simple UV mapping
    }

    // Fan triangulation (assumes convex polygon)
    for i in 1..points.len() - 1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::mesh::Indices::U32(indices));

    mesh
}

pub fn get_center_of_polygon(points: &[Vec2], angle: f32) -> Vec2{
    let mut center: Vec2 = Vec2::new(0.0, 0.0);
    for (i, point) in points.iter().enumerate(){
        if i == 0{center = *point;}
        else {center += point;}
    }

    return center/points.iter().count().val_num_f32();
}

pub fn rotate_polygon<const N: usize>(points: &[Vec2; N], angle: f32) -> [Vec2; N] {
    let mut rotated_points = *points;
    for i in 0..N {
        let point = &points[i];
        rotated_points[i].x = point.x * angle.cos() - point.y * angle.sin();
        rotated_points[i].y = point.x * angle.sin() + point.y * angle.cos();
    }
    return rotated_points;
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec2;
    
    const EPSILON: f32 = 0.0001;
    
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }
    
    fn approx_eq_vec2(a: Vec2, b: Vec2) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y)
    }

    #[test]
    fn test_create_polygon_mesh_triangle() {
        let triangle = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.5, 1.0),
        ];
        
        let mesh = create_polygon_mesh(&triangle);
        
        // Prüfe ob Mesh erstellt wurde
        assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
        assert!(mesh.indices().is_some());
    }

    #[test]
    fn test_create_polygon_mesh_rectangle() {
        let rectangle = vec![
            Vec2::new(-5.0, -5.0),
            Vec2::new(5.0, -5.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(-5.0, 5.0),
        ];
        
        let mesh = create_polygon_mesh(&rectangle);
        
        // Rectangle sollte 4 Vertices haben
        if let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            let vertex_count = match positions {
                bevy::mesh::VertexAttributeValues::Float32x3(data) => data.len(),
                _ => 0,
            };
            assert_eq!(vertex_count, 4);
        }
        
        // Rectangle sollte 2 Triangles haben (6 indices)
        if let Some(indices) = mesh.indices() {
            match indices {
                bevy::mesh::Indices::U32(data) => {
                    assert_eq!(data.len(), 6); // 2 triangles * 3 vertices
                }
                _ => panic!("Expected U32 indices"),
            }
        }
    }

    #[test]
    fn test_create_polygon_mesh_hexagon() {
        let hexagon = vec![
            Vec2::new(1.0, 0.0),
            Vec2::new(0.5, 0.866),
            Vec2::new(-0.5, 0.866),
            Vec2::new(-1.0, 0.0),
            Vec2::new(-0.5, -0.866),
            Vec2::new(0.5, -0.866),
        ];
        
        let mesh = create_polygon_mesh(&hexagon);
        
        // Hexagon: 6 vertices, 4 triangles (12 indices)
        if let Some(indices) = mesh.indices() {
            match indices {
                bevy::mesh::Indices::U32(data) => {
                    assert_eq!(data.len(), 12);
                }
                _ => panic!("Expected U32 indices"),
            }
        }
    }

    #[test]
    fn test_get_center_of_polygon_square() {
        let square = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
        ];
        
        let center = get_center_of_polygon(&square, 0.0);
        
        // Center sollte bei (0, 0) sein
        assert!(approx_eq_vec2(center, Vec2::new(0.0, 0.0)));
    }

    #[test]
    fn test_get_center_of_polygon_offset_rectangle() {
        let rectangle = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(4.0, 0.0),
            Vec2::new(4.0, 2.0),
            Vec2::new(0.0, 2.0),
        ];
        
        let center = get_center_of_polygon(&rectangle, 0.0);
        
        // Center sollte bei (2, 1) sein
        assert!(approx_eq_vec2(center, Vec2::new(2.0, 1.0)));
    }

    #[test]
    fn test_get_center_of_polygon_triangle() {
        let triangle = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(0.0, 3.0),
        ];
        
        let center = get_center_of_polygon(&triangle, 0.0);
        
        // Center = (0+3+0)/3, (0+0+3)/3 = (1, 1)
        assert!(approx_eq_vec2(center, Vec2::new(1.0, 1.0)));
    }

    #[test]
    fn test_rotate_polygon_90_degrees() {
        let points = [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ];
        
        let angle = std::f32::consts::PI / 2.0; // 90 degrees
        let rotated = rotate_polygon(&points, angle);
        
        // (1,0) rotated 90° -> (0,1)
        assert!(approx_eq_vec2(rotated[0], Vec2::new(0.0, 1.0)));
        // (0,1) rotated 90° -> (-1,0)
        assert!(approx_eq_vec2(rotated[1], Vec2::new(-1.0, 0.0)));
    }

    #[test]
    fn test_rotate_polygon_180_degrees() {
        let points = [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ];
        
        let angle = std::f32::consts::PI; // 180 degrees
        let rotated = rotate_polygon(&points, angle);
        
        // (1,0) rotated 180° -> (-1,0)
        assert!(approx_eq_vec2(rotated[0], Vec2::new(-1.0, 0.0)));
        // (0,1) rotated 180° -> (0,-1)
        assert!(approx_eq_vec2(rotated[1], Vec2::new(0.0, -1.0)));
    }

    #[test]
    fn test_rotate_polygon_360_degrees() {
        let points = [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 0.0),
        ];
        
        let angle = 2.0 * std::f32::consts::PI; // 360 degrees
        let rotated = rotate_polygon(&points, angle);
        
        // Nach 360° sollten Punkte wieder gleich sein
        for i in 0..3 {
            assert!(approx_eq_vec2(rotated[i], points[i]));
        }
    }

    #[test]
    fn test_rotate_polygon_zero_angle() {
        let points = [
            Vec2::new(5.0, 3.0),
            Vec2::new(-2.0, 7.0),
        ];
        
        let rotated = rotate_polygon(&points, 0.0);
        
        // Bei Winkel 0 sollten Punkte unverändert bleiben
        for i in 0..2 {
            assert!(approx_eq_vec2(rotated[i], points[i]));
        }
    }

    #[test]
    fn test_rotate_polygon_square() {
        let square = [
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
        ];
        
        let angle = std::f32::consts::PI / 4.0; // 45 degrees
        let rotated = rotate_polygon(&square, angle);
        
        // Prüfe dass alle Punkte rotiert wurden (nicht an Originalposition)
        let mut all_different = true;
        for i in 0..4 {
            if approx_eq_vec2(rotated[i], square[i]) {
                all_different = false;
            }
        }
        assert!(all_different);
    }

    #[test]
    fn test_create_polygon_mesh_normals_point_up() {
        let triangle = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.5, 1.0),
        ];
        
        let mesh = create_polygon_mesh(&triangle);
        
        if let Some(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
            if let bevy::mesh::VertexAttributeValues::Float32x3(data) = normals {
                // Alle Normals sollten nach oben zeigen (0, 0, 1)
                for normal in data {
                    assert!(approx_eq(normal[0], 0.0));
                    assert!(approx_eq(normal[1], 0.0));
                    assert!(approx_eq(normal[2], 1.0));
                }
            }
        }
    }
}
