# Docker

We provide pre-built Docker images with preconfigured School of Solana environments for different architectures.

> [!TIP]
> **Install Docker**
> 
> Follow this [installation guide](https://docs.docker.com/engine/install/).

## Quick Start

Choose the appropriate image for your system architecture:

### For x86/AMD64 Systems
```bash
docker pull ackeeblockchain/school-of-solana:latest
```

### For ARM Systems (Apple Silicon, ARM processors)
```bash
docker pull ackeeblockchain/school-of-solana-arm:latest
```

## Running the Container

> [!IMPORTANT]
> Replace `<image-name>` with either:
> - `ackeeblockchain/school-of-solana:latest` for x86/AMD64
> - `ackeeblockchain/school-of-solana-arm:latest` for ARM

**Create and run a new container:**
```bash
docker run -it --name school-of-solana -p 8899:8899 -p 9900:9900 -p 8000:8000 -p 8080:8080 <image-name>
```

**Access the development environment:**
Visit http://localhost:8080/

## Container Management

**Start an existing container:**
```bash
docker start school-of-solana
```

**Stop the container:**
```bash
docker stop school-of-solana
```

**Inspect Official Containers:** 

- [Official Docker Hub Images](https://hub.docker.com/u/ackeeblockchain) - If your curious, and want to inspect the official images, you can find them here.


> [!NOTE]
> You can also manage containers using Docker Desktop's GUI interface.
