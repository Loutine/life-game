use rand::Rng;
pub struct MapOfCell {
    x: usize,
    y: usize,
    pub cell_info:Vec<Vec<bool>>,
}

impl MapOfCell {
    pub fn new(x: i32, y: i32) -> Self {
	let x = x as usize;
	let y = y as usize;
        MapOfCell {
	    x ,
	    y ,
            cell_info: vec![vec![false; y ]; x],
        }
    }

    pub fn change_maps_to_true(self:&mut Self, tuple:(&usize, &usize), flag:bool) {
	let (x, y) = tuple;
	self.cell_info[*x][*y] = flag;
    }
    
    pub fn random(self:&mut Self, nums: u32) {
	let set_x:Vec<usize> = (0..nums)
	    .map
	    (|_|{
		rand::thread_rng().gen_range(0, self.x)
	    })
	    .collect();
	let set_y:Vec<usize> = (0..nums)
	    .map(|_|{
		rand::thread_rng().gen_range(0, self.y)
	    })
	    .collect();

	let cordin = set_x.iter().zip(set_y.iter());
	for cood in cordin {
	    self.change_maps_to_true(cood, true);
	}
    }

    pub fn nearby_state(self: &Self, x: i32, y: i32) -> Result<i32, String> {
	if x < 0 || y < 0 {
	    Err(String::from("not a positive number"))
	}else {
            let mut result: i32 = 0;
            let iterx: [i32; 3] = [-1, 0, 1];
            for i in iterx.iter() {
		for j in iterx.iter() {
                    if (i == &0 && j == &0) || x + i < 0 || y + j < 0 || x + i >= self.x as i32|| y + j >= self.y as i32{
			continue;
                    } else if self.cell_info[(x + i) as usize][(y + j) as usize] {
			result += 1;
                    }
		}
            }
            Ok(result)
	}
    }

    //need to recontribute use some interface to make it
    //try to use a draw trait to make it

    pub fn next(self: &mut MapOfCell) {
	let mut change_to_true:Vec<(usize, usize)> = vec![];
	let mut change_to_false:Vec<(usize, usize)> = vec![];
	for x in 0..self.x {
	    for y in 0..self.y{
		match self.nearby_state(x as i32, y as i32).unwrap() {
		    3 => change_to_true.push((x, y)),
		    2 => continue,
		    _ => change_to_false.push((x, y)),
		}
	    }
	}
	for (x,y) in change_to_true {
	    self.change_maps_to_true((&x, &y), true);
	}
	for (x,y) in change_to_false {
	    self.change_maps_to_true((&x, &y), false);
	}
    }
}
