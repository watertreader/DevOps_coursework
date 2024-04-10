
  <p align="center"><img src="juggle.png" /></p>
  <p align="center" style="color:blue;font-weight: bold;font-size:95px;">DevOps_tutorial</p> 

## Table of Contents

    Installation
    Project Motivation
    File Descriptions
    Results
    Acknowledgements
    Licensing, Authors, and Acknowledgements

## Installation

To be completed

### Requirements

#### Local Developement
<ul>
  <li> Rust Development Environment </li>
</ul>

#### Remote Developement
<ul>
  <li>  Docker </li>
</ul>

If you want to develop the application, you would need installation of an
 IDE 

###  Docker Setup
Building docker image in your terminal

    docker build -t rust_docker .

To run the Docker image, execute the following command:

    docker run -it --rm --name my-running-app rust_docker


## Project Motivation<a name="motivation"></a>

For this project, I am trying to document my journey in dipping into the devops world

You can also read up on my DevOps experience in Medium https://medium.com/@watertreader/dip-into-devops-world-9ad96f8ec774


## File Descriptions <a name="files"></a>

    READMD.md                    ---- readme file that you see right now
    Cargo.toml                   ---- Rust configuration and dependency file
    Dockerfile                   ---- Configure file to build the docker container
    juggle.png                   ---- image found in readme
    /src                         ---- source folder
    ---- main.rs                 ---- rust source file
    /resource                    ---- resource folder
    ---- docker_cheatsheet       ---- contain useful docker commands
    /scripts
    ---- 

## Results<a name="results"></a>

## Acknowledgements <a name="acknowedgement"></a>

Thanks to Uli Hiztel, my instructor and NUS Advanced Computing for Executive Center and of course, my company, ST Engineering Defence Aviation Services for sponsoring the course

It has been a "siong" journey!

## Licensing <a name="licensing"></a>

All rights reserved

All source code and software in this repository are made available under the terms of the GNU license.
