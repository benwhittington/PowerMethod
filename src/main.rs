use std::vec;
// use num_traits::pow;

#[allow(non_snake_case)]
#[allow(dead_code)]

struct EigPair {
    val: f32,
    vec: Vec<f32>,
}

impl EigPair {
    fn norm(&self) -> f32 {
        let mut total: f32=0.0;

        for v in self.vec.iter() {
            total+=v.powf(2.0);
        }

        return total.sqrt();
    }

    // the reason this takes a value norm is because otherwise this is 
    // computed twice elsewhere
    fn normalise(&mut self,norm: f32) {
        for v in self.vec.iter_mut() {
            *v/=norm;
        }
    }
}

// NOTE: Modifies input vector b
fn DotProduct(A: &Vec<Vec<f32>>, b: &mut Vec<f32>) {
    let com_dim=A.len();
    let out_dim=A[0].len();
    assert_eq!(com_dim,b.len()); // cols of A==rows of b

    let temp: Vec<f32>=b.to_vec();

    for i in 0..out_dim{
        
        b[i]=0.;
        for j in 0..com_dim{
            b[i]+=A[i][j] * temp[j];
        }
    }

    // return out;
}

// todo this is rad
// ! this is rad
// ? rad
// * this is rad
//// this is not rad 
// this is rad


fn PowerMethod(A: &Vec<Vec<f32>>, v: &Vec<f32>, tol: f32) -> EigPair {
    let mut eig = EigPair {val: 1.0, vec: v.to_vec()};
    let mut prev: f32=0.;

    eig.val=eig.norm();

    while ((eig.val - prev)/(1.+prev)).abs()>tol {
        eig.normalise(eig.val);
        prev=eig.val;
        DotProduct(&A, &mut eig.vec);
        eig.val=eig.norm();
    }

    return eig;
}

fn deflate(A: &mut Vec<Vec<f32>>, eig: &mut EigPair) {
    let n: usize=A.len();
    let mut norm_sqd=eig.norm();
    norm_sqd*=norm_sqd;
    // eig.normalise(norm_sqd);
    
    for i in 0..n {
        for j in 0..n {
            A[i][j]-=eig.val*eig.vec[i]*eig.vec[j]/norm_sqd;
        }
    }
}

fn get_eigs(A: &mut Vec<Vec<f32>>) -> Vec<EigPair> {
    let mut eigpairs: Vec<EigPair>=Vec::new();
    let n: usize=A.len();
    let v: Vec<f32>=vec![2.;n]; // initial guess
    let tol: f32=1e-6;

    for _ in 0..n {
        eigpairs.push(PowerMethod(&A, &v, tol));
        deflate(A, &mut eigpairs.last_mut().unwrap());
        println!("{:?}",A);
    }
    

    return eigpairs
}

fn main() {
    let mut A=vec![
        vec![6.,-1.],
        vec![2.0,3.0]
    ];
    let mut b: Vec<f32> =vec![2.0,1.0];

    // let mut eig: EigPair;

    // DotProduct(&A,&mut b);
    // println!("{:?}",b);

    // eig=PowerMethod(&A, &b, 1e-6);
    let eigs: Vec<EigPair>;

    eigs=get_eigs(&mut A);

    for eig in eigs.iter() {
        println!("{}",eig.val);
    }

}

