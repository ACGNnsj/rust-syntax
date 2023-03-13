use sprs::{CsMat, TriMat};

#[link(name = "spsolver")]
extern "C" {
    fn spsolver_LU(A: *const CdefMatrix, A_size: i32, B: *const CdefVector, B_size: i32, row: i32, col: i32) -> *mut CdefRetVector;
}


#[repr(C)]
pub struct CdefMatrix {
    row: i32,
    col: i32,
    data: f64,
}

#[repr(C)]
#[derive(Debug)]
pub struct CdefVector {
    row: i32,
    data: f64,
}

#[repr(C)]
pub struct CdefRetVector {
    size: i32,
    data: *mut CdefVector,
}

fn sparse_to_cdef_matrix(x: &CsMat<f64>) -> Vec<CdefMatrix> {
    let mut ret = vec![];
    for (&v, (row, col)) in x.iter() {
        ret.push(CdefMatrix { row: row as i32, col: col as i32, data: v });
    }

    ret
}

fn sparse_to_cdef_vector(x: &CsMat<f64>) -> Vec<CdefVector> {
    let mut ret = vec![];
    for (&v, (row, _)) in x.iter() {
        ret.push(CdefVector { row: row as i32, data: v });
    }

    ret
}

fn cdef_vector_to_sparse(x: Vec<CdefVector>, nrows: usize, ncols: usize) -> CsMat<f64> {
    let mut tri = TriMat::new((nrows, 1));
    for vector in x.iter() {
        tri.add_triplet(vector.row as usize, 0, vector.data);
    }
    tri.to_csc()
}

pub fn spsolver_lu(a: &CsMat<f64>, b: &CsMat<f64>) -> CsMat<f64> {
    let rows = a.rows() as i32;
    let cols = a.cols() as i32;
    let a = sparse_to_cdef_matrix(a);
    let b = sparse_to_cdef_vector(b);
    let a_len = a.len() as i32;
    let b_len = b.len() as i32;
    unsafe {
        let x = spsolver_LU(a.as_ptr(), a_len, b.as_ptr(), b_len, rows, cols);
        // rebuild results as Vec<cdef_veoctr>
        let x_len = (*x).size as usize;
        let x_cap = x_len;
        let x_ptr = (*x).data;
        let rebuilt_x = Vec::from_raw_parts(x_ptr, x_len, x_cap);
        cdef_vector_to_sparse(rebuilt_x, rows as usize, cols as usize)
    }
}

#[test]
fn test() {
    println!("Hello, world!");
    let a: CsMat<f64> = CsMat::eye(3);
    let b: CsMat<f64> = CsMat::eye(3);
    let c = spsolver_lu(&a, &b);
    println!("{:?}", c);
}