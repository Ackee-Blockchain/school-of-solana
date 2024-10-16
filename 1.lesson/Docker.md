# Docker

- [Docker Image x86](#docker-image-x86)
- [Docker Image ARM](#docker-image-arm)

## Docker Image x86

> [!TIP]
> **Install Docker**
> You can check this guide [Install Docker Engine](https://docs.docker.com/engine/install/)


> [!IMPORTANT]
> To use the pre-built Docker image for this course, you can pull the image from Docker Hub:
> ```bash
> docker pull ackeeblockchain/school-of-solana:latest
> ```
> Then run the following command. This will create new container.
> ```bash
> docker run -it --name school-of-solana -p 8899:8899 -p 9900:9900 -p 8000:8000 -p 8080:8080 ackeeblockchain/school-of-solana:latest
> ```
> Then visit the following url
>
>   http://localhost:8080/
>
> When you want to continue work inside the Docker Image, use
> ```bash
> docker start school-of-solana
> ```
> and again visit the URL.
> To stop the Image, use
> ```bash
> docker stop school-of-solana
> ```
> It is also possible to start/stop the Imagre from the Docker Desktop GUI.

## Docker Image ARM

> [!TIP]
> **Install Docker**
> You can check this guide [Install Docker Engine](https://docs.docker.com/engine/install/)


> [!IMPORTANT]
> To use the pre-built Docker image for this course, you can pull the image from Docker Hub:
> ```bash
> docker pull ackeeblockchain/school-of-solana-arm:latest
> ```
> Then run the following command. This will create new container.
> ```bash
> docker run -it --name school-of-solana -p 8899:8899 -p 9900:9900 -p 8000:8000 -p 8080:8080 ackeeblockchain/school-of-solana-arm:latest
> ```
> Then visit the following url
>
>   http://localhost:8080/
>
> When you want to continue work inside the Docker Image, use
> ```bash
> docker start school-of-solana
> ```
> and again visit the URL.
> To stop the Image, use
> ```bash
> docker stop school-of-solana
> ```
> It is also possible to start/stop the Imagre from the Docker Desktop GUI.

-----
