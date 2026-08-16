# Math
The maths functions and structures are design for any number of dimensions and data types. Generally mathenatical concepts are based on principles, axioms and relationships. As long as those are maintained, the mathematics are generally sound. So instead of implementing multiple versions of the same code for diffirenting number of dimensions and data types, only a single generic base type is implemented.

## Point
A point is a fixed location in N-dimensional space. In this library it is expressed as:

> `Point<T,N>`

**where:**
* `N` is the number of dimensions in space. I.e. 1 for 1D, 2 for 2D, 3 for 3D, etc.
* `T` is the storage type to use for the dimensional elements in the Point. 

## Vector
A vector is a direction in N-dimensional space. In this library it is expressed as:

> `Vector<T,N>`

**where:**
* `N` is the number of dimensions in space. I.e. 1 for 1D, 2 for 2D, 3 for 3D, etc.
* `T` is the storage type to use for the dimensional elements in the Point. 

## Matrix

> `Matrix<T,M,N>`

**where:**
* `N` the number of columns in the matrix.
* `M` the number of rows in the matrix.
* `T` the storage type of the matrix elements.