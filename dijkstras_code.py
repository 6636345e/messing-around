
from pyspark.sql import SparkSession #from pyspart.sql library import SpartkSession


#Making Spark build enviromnent
spark = SparkSession.builder\
	.master("local").appName("hbdatas_app").getOrCreate()

bdata=spark.read.json("hdfs://localhost:9000/InputData/yelp_academic_dataset_business.json") #reading the dataset from the HDFS file system




#Prunning dataset to have Philadelphia restaurants which are 4 star-rated only
pr = bdata.filter((bdata.stars >= 4) & (bdata.hours.isNotNull()) & (bdata.categories.contains("Restaurants") & (bdata.city == "Philadelphia")))
pr = pr.filter(pr.hours.Monday.isNotNull() & 
               pr.hours.Tuesday.isNotNull() & 
               pr.hours.Wednesday.isNotNull() & 
               pr.hours.Thursday.isNotNull() & 
               pr.hours.Friday.isNotNull() & 
               pr.hours.Saturday.isNotNull() &
               pr.hours.Sunday.isNotNull())      
pr.show() #show the dataset of Philadelphia restaurants which are 4 star-rated