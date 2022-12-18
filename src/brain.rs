const SIZE: usize = 32;

struct Neuron {
    weight: f32,
    x: f32,
    y: f32,
    z: f32,
}

struct Brain {
    field: [[[Neuron; SIZE]; SIZE]; SIZE],
    energy: [[[f32; SIZE]; SIZE]; SIZE],
}

impl Clone for Brain {
    fn clone(&self) -> Self {
        Brain {
            field: self.field.clone(),
        }
    }
}

impl Brain {
    fn new() -> Brain {
        Brain {
            field: [[[Neuron {
                weight: 0.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }; SIZE]; SIZE]; SIZE],
            energy: [[[0.0; SIZE]; SIZE]; SIZE],
        }
    }

    fn mutate(&mut self) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    self.field[x][y][z].weight = rand::random::<f32>() * 2.0 - 1.0;
                    self.field[x][y][z].x = rand::random::<f32>() * 2.0 - 1.0;
                    self.field[x][y][z].y = rand::random::<f32>() * 2.0 - 1.0;
                    self.field[x][y][z].z = rand::random::<f32>() * 2.0 - 1.0;
                }
            }
        }
    }
}
