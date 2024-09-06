# Free-form Floor Plan Design using Differentiable Voronoi Diagram

![](https://github.com/nobuyuki83/floor_plan/blob/images/teaser.png?raw=true)



|[Paper PDF](https://www.dropbox.com/scl/fi/culi7j1v14r9ax98rfmd6/2024_pg24_floorplan.pdf?rlkey=s5xwncuybrtsj5vyphhn61u0h&dl=0)|



## Publication

Xuanyu Wu, Kenji Tojo, Nobuyuki Umetani, "Free-form Floor Plan Design using Differentiable Voronoi Diagram," Pacific Graphics 2024 proceedings 



## Abstract

Designing floor plans is difficult because various constraints must be satisfied by the layouts of the internal walls. This paper presents a novel shape representation and optimization method for designing floor plans based on the Voronoi diagrams. Our Voronoi diagram implicitly specifies the shape of the room using the distance from the Voronoi sites, thus facilitating the topological changes in the wall layout by moving these sites. Since the differentiation of the explicit wall representation is readily available, our method can incorporate various constraints, such as room areas and room connectivity, into the optimization. We demonstrate that our method can generate various floor plans while allowing users to interactively change the constraints.



## How to run

The demos are written in `Rust`. If you don't have Rust on your computer, please install the Rust development environment. Here is the list of commands that generate GIF animations of convergence.

The command ```run --example 0_shapeA --release``` results in following animations (left: random seed = 0, right: random seed = 1)

![](https://github.com/nobuyuki83/floor_plan/blob/images/0_shapeA_0.gif?raw=true)  ![](https://github.com/nobuyuki83/floor_plan/blob/images/0_shapeA_1.gif?raw=true)


----

The command ```run --example 1_shapeB --release``` results in following animations  (left: random seed = 0, right: random seed = 1)

![](https://github.com/nobuyuki83/floor_plan/blob/images/1_shapeB_0.gif?raw=true)  ![](https://github.com/nobuyuki83/floor_plan/blob/images/1_shapeB_1.gif?raw=true)

---

The command ```run --example 2_shapeC --release``` results in following animations  (left: random seed = 4, right: random seed = 7)

![](https://github.com/nobuyuki83/floor_plan/blob/images/2_shapeC_4.gif?raw=true)  ![](https://github.com/nobuyuki83/floor_plan/blob/images/2_shapeC_7.gif?raw=true)


----

The command ```run --example 3_duck --release``` results in following animations (left: random seed = 0, right: random seed = 5)

![](https://github.com/nobuyuki83/floor_plan/blob/images/3_duck_0.gif?raw=true)  ![](https://github.com/nobuyuki83/floor_plan/blob/images/3_duck_5.gif?raw=true)



