import * as cdk from 'aws-cdk-lib';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';
import { aws_lambda as lambda } from "aws-cdk-lib";
import * as path from 'path';

export class CdkLocalstackDemoStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        new s3.Bucket(this, 'MyLocalBucket', {
            bucketName: 'my-local-bucket',
            removalPolicy: cdk.RemovalPolicy.DESTROY,
        });

        // Create a Lambda function
        new lambda.Function(this, 'HelloLambda', {
            runtime: lambda.Runtime.NODEJS_18_X,
            handler: 'hello.handler',
            code: lambda.Code.fromAsset(path.join(__dirname, 'lambda')),
            functionName: 'hello-lambda',
            description: 'A simple Lambda function that returns a greeting',
            memorySize: 128,
            timeout: cdk.Duration.seconds(10),
        });


    }
}
