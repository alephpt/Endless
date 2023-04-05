
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    // procedurally generate 'asteroid' mesh
    pub fn asteroid() -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut rng = rand::thread_rng();
        let mut offset = 0;

        for _ in 0..100 {
            let mut polygon = Vec::new();
            let mut polygon_indices = Vec::new();

            let mut last = Vector2::new(0.0, 0.0);
            for _ in 0..rng.gen_range(3, 7) {
                let mut v = Vector2::new(
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                );
                v = v.normalize() * rng.gen_range(0.0, 1.0);
                polygon.push(v);
            }

            for i in 0..polygon.len() {
                let v = Vertex {
                    position: [polygon[i].x, polygon[i].y, 0.0],
                    color: [1.0, 1.0, 1.0],
                };
                vertices.push(v);
                polygon_indices.push(offset + i as u32);
            }

            for i in 0..polygon.len() {
                let i1 = i;
                let i2 = (i + 1) % polygon.len();
                let i3 = (i + 2) % polygon.len();

                let v1 = polygon[i1];
                let v2 = polygon[i2];
                let v3 = polygon[i3];

                let a = (v2 - v1).normalize();
                let b = (v3 - v2).normalize();

                if a.dot(b) < 0.95 {
                    indices.push(polygon_indices[i1]);
                    indices.push(polygon_indices[i2]);
                    indices.push(polygon_indices[i3]);
                }
            }

            offset += polygon.len() as u32;
        }

        Self::new(vertices, indices)
    } 

    // procedurally generate pentagonal pyramid 'ship' mesh
    pub fn ship() -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut offset = 0;

        let mut polygon = Vec::new();
        let mut polygon_indices = Vec::new();

        let mut last = Vector2::new(0.0, 0.0);
        for _ in 0..5 {
            let mut v = Vector2::new(
                rand::thread_rng().gen_range(-1.0, 1.0),
                rand::thread_rng().gen_range(-1.0, 1.0),
            );
            v = v.normalize() * rand::thread_rng().gen_range(0.0, 1.0);
            polygon.push(v);
        }

        for i in 0..polygon.len() {
            let v = Vertex {
                position: [polygon[i].x, polygon[i].y, 0.0],
                color: [1.0, 1.0, 1.0],
            };
            vertices.push(v);
            polygon_indices.push(offset + i as u32);
        }

        for i in 0..polygon.len() {
            let i1 = i;
            let i2 = (i + 1) % polygon.len();
            let i3 = (i + 2) % polygon.len();

            let v1 = polygon[i1];
            let v2 = polygon[i2];
            let v3 = polygon[i3];

            let a = (v2 - v1).normalize();
            let b = (v3 - v2).normalize();

            if a.dot(b) < 0.95 {
                indices.push(polygon_indices[i1]);
                indices.push(polygon_indices[i2]);
                indices.push(polygon_indices[i3]);
            }
        }

        offset += polygon.len() as u32;

        Self::new(vertices, indices)
    }

    // procedurally generate 'bullet' mesh
    pub fn bullet() -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut offset = 0;

        let mut polygon = Vec::new();
        let mut polygon_indices = Vec::new();

        let mut last = Vector2::new(0.0, 0.0);
        for _ in 0..5 {
            let mut v = Vector2::new(
                rand::thread_rng().gen_range(-1.0, 1.0),
                rand::thread_rng().gen_range(-1.0, 1.0),
            );
            v = v.normalize() * rand::thread_rng().gen_range(0.0, 1.0);
            polygon.push(v);
        }

        for i in 0..polygon.len() {
            let v = Vertex {
                position: [polygon[i].x, polygon[i].y, 0.0],
                color: [1.0, 1.0, 1.0],
            };
            vertices.push(v);
            polygon_indices.push(offset + i as u32);
        }

        for i in 0..polygon.len() {
            let i1 = i;
            let i2 = (i + 1) % polygon.len();
            let i3 = (i + 2) % polygon.len();

            let v1 = polygon[i1];
            let v2 = polygon[i2];
            let v3 = polygon[i3];

            let a = (v2 - v1).normalize();
            let b = (v3 - v2).normalize();

            if a.dot(b) < 0.95 {
                indices.push(polygon_indices[i1]);
                indices.push(polygon_indices[i2]);
                indices.push(polygon_indices[i3]);
            }
        }

        offset += polygon.len() as u32;

        Self::new(vertices, indices)
    }
}