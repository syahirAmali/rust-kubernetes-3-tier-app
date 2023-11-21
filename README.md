# Kubernetes-3-Tier-App

- Built with rust, using rocket, diesel and rdkafka crates for a basic webservice with kafka.
- The kafka is configured to have 2 topics for each of the api call, which is for counter and config, while there is only 1 producer and 1 consumer.
- Database uses a simple postgresql setup and runs along side the webservice and static frontend.
- The static frontend can be found in the static folder under src.
- It is deployed on GKE with kubernetes.
- The application is grouped into the docker-compose and built seperately and deployed on Kubernetes with the respective yaml files.
- There is only 1 node for kubernetes since we're not expecting much load for this.

## Things that could be improved on

- Due to time limitations Terraform and Ansible was not set up for this test.
- TLS was also not implemented, this should be done much easily if an existing framework was used, but to simplify and speed up dev process it wasnt implemented.
- A proper authentication should be implemented for the webservice to further enchance security and stop traversal attacks, this can be done with OAuth2.
- There is also no rate limits implemented for the post functions. This should also be implemented to stop malicious bot activity.
