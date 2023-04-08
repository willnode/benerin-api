#!/usr/bin/env python3

import csv
import os
from tqdm import tqdm
import mysql.connector
from dotenv import load_dotenv

os.chdir(__file__[:__file__.rfind('/')])
load_dotenv()

mydb = mysql.connector.connect(
    host=os.getenv('DB_HOST'),
    user=os.getenv('DB_USER'),
    password=os.getenv('DB_PASS'),
    database=os.getenv('DB_NAME')
)

mycursor = mydb.cursor()
mycursor.execute("TRUNCATE kbbi")
with open('3-combword.csv', 'r', encoding='utf8') as f:
    reader = csv.reader(f)
    batch = []
    def addbatch():
        mycursor.executemany("INSERT IGNORE kbbi (word, bword, kind, notes) VALUES (%s, %s, %s, %s)", batch)
        batch.clear()
    for row in tqdm(reader):
        batch.append(row)
        if len(batch) == 5000:
            addbatch()
    addbatch()
    mydb.commit()
