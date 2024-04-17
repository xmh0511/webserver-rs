### Feature
An out-of-the-box web server that is encapsulated based on salvo. 

### Use DataBase (sea-orm)
1. Enable one of the following features, depending on what you need
> - mysql 
> - sqlite
> - postgres


2. Run the following command if you want to use database
> sea-orm-cli generate entity -o src/model

3. Import the generated model in your `main.rs` file
> mod model;

### Use Http3
Enable `http3` feature  