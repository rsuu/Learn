fn main() {
    iter_struct();
}

fn iter_struct() {
    let vec3 = Vector3D {
        x: 3.0,
        y: 7.0,
        z: 22.0,
    };
    for a in vec3.iter() {
        dbg!(a);
    }
}

struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3D {
    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }
}

struct Iter<'a> {
    inner: &'a Vector3D,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a f32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.x,
            1 => &self.inner.y,
            2 => &self.inner.z,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

pub fn vec_to_string_fill(i: &Vec<String>, c: &str) -> String {
    i.into_iter().fold(String::new(), |mut s, f| {
        s.push_str(&f);
        s.push_str(c);
        s
    })
}
