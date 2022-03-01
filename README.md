[![Contributors][contributors-shield]][contributors-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- ABOUT THE PROJECT -->
# rust-web
This is a web project using Rust, Actix-web, Yugabyte, Diesel


## Steps to Use the Project
1. Install the docker into your machine from this site [https://docs.docker.com/engine/install/](https://docs.docker.com/engine/install/)
2. Pull the Yugabyte container using docker by this command _**docker pull yugabytedb/yugabyte**_
3. Run the Yugabyte docker container using this command: \
   _**docker run -d --name yugabyte -p7000:7000 -p9000:9000 -p5433:5433 -p9042:9042 -v ~/yb_data:/home/yugabyte/yb_data yugabytedb/yugabyte:latest bin/yugabyted start --base_dir=/home/yugabyte/yb_data --daemon=false**_
4. Ensure that the image has been run by this command _**sudo docker ps -a**_, you will find the image name, container id and some other options
5. Install the cargo-swagger into the project and use the extracted yaml file into this site [https://editor.swagger.io/](https://editor.swagger.io/) to see all endpoints with example, and the model in more details.
6. Run the Server from the main file and try to use the endpoints from the swagger site.


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/othneildrew/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://github.com/AbdelazizSaid250
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/abdelaziz-said-4a9b12127