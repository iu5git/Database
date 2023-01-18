import csv
import io

from db_processor import DBConection

def readdb(dbconn:DBConection, message:str):
    # buf = dbconn.read_drom_db(message).to_records(index=False)
    s = io.StringIO()
    csv.writer(s).writerows(dbconn.read_drom_db(message).to_records(index=False))
    s.seek(0)
    buf = io.BytesIO()
    buf.write(s.getvalue().encode())
    buf.seek(0)
    buf.name = f'out.csv'
    return buf

def writedb(dbconn:DBConection, message:str):
    # Вот тут возможно потребцется поменять логику в зависимости от ващей базы
    vals = message.split('| ')
    return dbconn.write_to_db(vals[0], (tuple(int(i) for i in vals[1:])))


def deletedb(dbconn:DBConection, message:str):
    return dbconn.delete_drom_db(message)