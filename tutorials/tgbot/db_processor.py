import pandas as pd
import psycopg2
from psycopg2.extras import execute_batch


class DBConection:
    def __init__(self, host:str, port:str, db_name:str, user:str, password:str) -> None:
        self.connect_config = {
            "host": host,
            "port": port, 
            "dbname": db_name,
            "user": user, 
            "password": password
        }

    def read_drom_db(self, sql_query:str) -> pd.DataFrame:
        '''
        Написать функцию для чтения данных из базы в pandas DataFrame
        '''
        # conn = psycopg2.connect(**self.connect_config)
        # df = pd.read_sql(sql_query, conn)
        # conn.close()
        # return df
        pass

    def delete_drom_db(self, sql_query:str) -> str:
        '''
        Написать функцию для удаления данных из базу
        '''
        # try:
        #     conn = psycopg2.connect(**self.connect_config)
        #     cursor = conn.cursor()
        #     cursor.execute(sql_query)
        #     conn.commit()
        #     cursor.close()
        #     conn.close()
        #     return 'Done'
        # except Exception as exp:
        #     return exp
        pass

    def write_to_db(self, sql_query:str, valuses:tuple) -> str:
        '''
        Написать функцию для записи данных в базу
        '''
        # try:
        #     conn = psycopg2.connect(**self.connect_config)
        #     cursor = conn.cursor()
        #     cursor.execute(sql_query, valuses)
        #     conn.commit()
        #     cursor.close()
        #     conn.close()
        #     return 'Done'
        # except Exception as exp:
        #     return exp
        pass