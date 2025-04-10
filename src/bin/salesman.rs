
struct Points {
	x: int16,
	y: int16
}

fn p2p(from: Points, to: Points) -> float64 {
	let xd: float64 = from.x - to.x;
	let yd: float64 = from.y - to.y;
	d = float64(xd*xd) + float64(yd*yd);
	d = math.Sqrt(d);
	return
}

fn dist(idx: &[int], points: &[Points]) -> float64 {
	sum = 0;
	for i in 1..len(idx) {
		sum += p2p(points[idx[i-1]], points[idx[i]])
	}
	sum += p2p(points[idx[0]], points[idx[len(idx)-1]]);
	return sum;
}

fn main() {
	let idx : [i32; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

	points := []Points{
		Points{1, 1},
		Points{2, 1},
		Points{1, 2},
		Points{2, 2},
		Points{3, 3},
		Points{4, 3},
		Points{3, 4},
		Points{4, 4},
		Points{2, 4},
		Points{3, 0},
		Points{4, 1},
		Points{2, 3},
	}

	min := dist(idx[:], points[:])

	size := len(idx)
	p := make([]int, size+1)
	for i := 0; i < size+1; i++ {
		p[i] = i
	}
	for i := 1; i < size; {
		p[i]--
		j := 0
		if i%2 == 1 {
			j = p[i]
		}
		tmp := idx[j]
		idx[j] = idx[i]
		idx[i] = tmp
		d := dist(idx[:], points[:])
		if d < min {
			min = d
		}
		for i = 1; p[i] == 0; i++ {
			p[i] = i
		}
	}

	fmt.Println(min)
}