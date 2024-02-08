"""
Esse programa gera a pasta de release do projeto para fazer o pull request no repositório da Rinha

A pasta será gerada dentro da pasta entregas/ e conterá:

- O arquivo docker-compose.yml, com as imagens dos serviços
- O arquivo README.md, com as informações do projeto
- A pasta migrations/, com as migrações do banco de dados
- A pasta nginx/, com as configurações do proxy reverso
"""
import shutil
from datetime import datetime
from pathlib import Path

import yaml

ROOT = Path(__file__).parent
ENTREGAS_FOLDER = (ROOT / "entregas")
DOCKER_COMPOSE_FILE = (ROOT / "docker-compose.yml")
README_MD_FILE = (ROOT / "README.md")
MIGRATIONS_FOLDER = (ROOT / "migrations")
NGINX_FOLDER = (ROOT / "nginx")

if __name__ == "__main__":
    # Ensure entregas/ folder exists
    ENTREGAS_FOLDER.mkdir(exist_ok=True)

    # Create a new folder for the release
    timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    RELEASE_FOLDER = ENTREGAS_FOLDER / f"cadu-{timestamp}"
    RELEASE_FOLDER.mkdir(exist_ok=False)

    # Read DOCKER_COMPOSE_FILE and customize it
    # - remove the "build" key from each service, as we'll be using the images from Docker Hub
    with open(DOCKER_COMPOSE_FILE, "r") as f:
        docker_compose_yaml = yaml.safe_load(f)

        for service in docker_compose_yaml["services"]:
            if "build" in docker_compose_yaml["services"][service]:
                del docker_compose_yaml["services"][service]["build"]

        # Write the customized docker-compose.yml to the release folder
        (RELEASE_FOLDER / "docker-compose.yml").write_text(yaml.dump(docker_compose_yaml))

    # Copy the README.md.readme to the release folder as README.md
    shutil.copy(README_MD_FILE, RELEASE_FOLDER / "README.md")

    # Copy the migrations folder
    shutil.copytree(MIGRATIONS_FOLDER, RELEASE_FOLDER / "migrations")

    # Copy the nginx folder
    shutil.copytree(NGINX_FOLDER, RELEASE_FOLDER / "nginx")


print(ROOT)
