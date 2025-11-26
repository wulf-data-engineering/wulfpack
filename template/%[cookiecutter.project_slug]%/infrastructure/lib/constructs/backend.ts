import {Construct} from 'constructs';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import * as cognito from "aws-cdk-lib/aws-cognito";
import {Api} from "./backend/api";
import {Identity} from "./backend/identity";
import {DeploymentConfig} from "../config";

interface BackendProps {
    config: DeploymentConfig;
}

/**
 * Sets up the backend resources.
 * Exposes the /api entrypoint for CloudFront.
 * Exposes the Cognito User Pool for authentication from frontend.
 */
export class Backend extends Construct {
    public readonly restApi?: apigateway.RestApi;
    public readonly userPool?: cognito.UserPool;
    public readonly userPoolClient?: cognito.UserPoolClient;

    constructor(scope: Construct, id: string, props: BackendProps) {
        super(scope, id);

        const deploymentConfig = props.config;

        // Locally cognito-local and cargo lambda watch are used instead
        if (deploymentConfig.aws) {

            const identity = new Identity(this, 'Identity', {deploymentConfig});

            this.userPool = identity.userPool;
            this.userPoolClient = identity.userPoolClient;

            const api = new Api(this, 'Api', {deploymentConfig, userPool: this.userPool});
            this.restApi = api.gateway;
        }
    }
}
