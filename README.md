
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

   <ol>
      <li> Installed  <a href="https://code.visualstudio.com/download">VS Code</a> from microsoft website  </li>
      <li> Installed any extension you need with <a href="https://code.visualstudio.com/docs/languages/rust"> Rust </a> </li?
      <li> Download or clone the code from Github </li>
      <li> <Optional> Install <a href="https://docs.docker.com/desktop/">Docker Desktop</a> if you need remote development </li>
      <li> You can start your development </li>
    </ol>

### Requirements

   #### Local Developement
   <ul>
     <li>  Rust Development Environment </li>
     <li> Suitable IDE (VS Code) </li>
   </ul>
   
   #### Remote Developement
   <ul>
     <li>  Docker Desktop </li>
     <li> Consider also using VS Code Remote Container extension </li>
   </ul>

###  Docker Setup
Building docker image in your terminal

    docker build -t rust_docker .

To run the Docker image, execute the following command:

    docker run -it --rm --name my-running-app rust_docker


## Project Motivation<a name="motivation"></a>

For this project, I am trying to document my journey in dipping into the devops world

You can also read up on my DevOps experience in  <a href="https://medium.com/@watertreader/dip-into-devops-world-9ad96f8ec774">Medium</a>


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

For local development, 
<ol>
  <li>Build the project by running cargo build </li>
  <li>Type in cargo run at the project directory after building </li>
  <li> Launch the browser at the url http://127.0.0.1:4040 </li>
  <li> You can view the current time and temperature of Singapore </li>
</ol>
Alternatively, you can run the script local.sh at the script folder
<br>
<br>

The result

  <p align="center"><img src="resource/Screenshot 2024-04-13 105248.png" /></p>

For remote development (docker)
<ol>
  <li> You can execute your container build at </li>
  <li> After building the docker image, you can run it by docker run -p 3000:4040 -env_file=<your file key> devops_rust </li>
   <li> Launch the browser at the url http://127.0.0.1:3000 </li>
  <li> You can view the current time and temperature of Singapore </li>
  </ol>
Alternatively, you can run the script remote.sh within the script folder

 <p align="center"><img src="resource/Screenshot 2024-04-13 110537.png" /></p>

## Acknowledgements <a name="acknowedgement"></a>

Thanks to Uli Hiztel, my instructor and NUS Advanced Computing for Executive Center 

It has been a "siong" journey!

## Licensing <a name="licensing"></a>

All rights reserved

All source code and software in this repository are made available under the terms of the GNU license.
