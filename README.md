# Free-form Floor Plan Design using Differentiable Voronoi Diagram

![](https://github.com/nobuyuki83/floor_plan/blob/images/teaser.png?raw=true)



|[Paper PDF](https://www.dropbox.com/scl/fi/ohj2uzvg12fejukkffw0q/2024_pg24_floorplan.pdf?rlkey=8magkoslj77d5o31a7zto01mt&dl=0)|



## Publication

Xuanyu Wu, Kenji Tojo, Nobuyuki Umetani, "Free-form Floor Plan Design using Differentiable Voronoi Diagram," Pacific Graphics 2024 proceedings 



## Abstract

Designing floor plans is difficult because various constraints must be satisfied by the layouts of the internal walls. This paper presents a novel shape representation and optimization method for designing floor plans based on the Voronoi diagrams. Our Voronoi diagram implicitly specifies the shape of the room using the distance from the Voronoi sites, thus facilitating the topological changes in the wall layout by moving these sites. Since the differentiation of the explicit wall representation is readily available, our method can incorporate various constraints, such as room areas and room connectivity, into the optimization. We demonstrate that our method can generate various floor plans while allowing users to interactively change the constraints.



## How to run

The demos are written in `Rust`. If you don't have Rust on your computer, please install the Rust development environment.

```bash
run --example 0_shapeA --release
```

![](https://github.com/nobuyuki83/floor_plan/blob/images/0_shapeA_0.gif?raw=true)  ![](https://github.com/nobuyuki83/floor_plan/blob/images/0_shapeA_1.gif?raw=true)



