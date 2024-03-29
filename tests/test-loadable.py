import sqlite3
import unittest

EXT_PATH="./target/debug/libbasesixfour0"

def connect(ext):
  db = sqlite3.connect(":memory:")

  db.execute("create table base_functions as select name from pragma_function_list")
  db.execute("create table base_modules as select name from pragma_module_list")

  db.enable_load_extension(True)
  db.load_extension(ext)

  db.execute("create temp table loaded_functions as select name from pragma_function_list where name not in (select name from base_functions) order by name")
  db.execute("create temp table loaded_modules as select name from pragma_module_list where name not in (select name from base_modules) order by name")

  db.row_factory = sqlite3.Row
  return db


db = connect(EXT_PATH)

def explain_query_plan(sql):
  return db.execute("explain query plan " + sql).fetchone()["detail"]

def execute_all(sql, args=None):
  if args is None: args = []
  results = db.execute(sql, args).fetchall()
  return list(map(lambda x: dict(x), results))

FUNCTIONS = [
  "base64_debug",
  "base64_decode",
  "base64_encode",
  "base64_version",  
]

MODULES = []

class TestBase64(unittest.TestCase):
  def test_funcs(self):
    funcs = list(map(lambda a: a[0], db.execute("select name from loaded_functions").fetchall()))
    self.assertEqual(funcs, FUNCTIONS)

  def test_modules(self):
    modules = list(map(lambda a: a[0], db.execute("select name from loaded_modules").fetchall()))
    self.assertEqual(modules, MODULES)
    
  def test_base64_version(self):
    version = 'v0.1.0'
    self.assertEqual(db.execute("select base64_version()").fetchone()[0], version)
  
  def test_base64_debug(self):
    debug = db.execute("select base64_debug()").fetchone()[0]
    self.assertEqual(len(debug.splitlines()), 2)
  
  def test_base64_decode(self):
    base64_decode = lambda content: db.execute("select base64_decode(?)", [content]).fetchone()[0]
    self.assertEqual(base64_decode('YWxleA=='), 'alex')
    #with self.assertRaisesRegex(sqlite3.OperationalError, "Unexpected null value"):
    #  base64_decode(None)

    with self.assertRaisesRegex(sqlite3.OperationalError, "error decoding: Invalid byte 32, offset 3."):
      base64_decode("not base64") 
    
    with self.assertRaisesRegex(sqlite3.OperationalError, "error decoding: Encoded text cannot have a 6-bit remainder."):
      base64_decode("not base6") 
  
  def test_base64_encode(self):
    base64_encode = lambda content: db.execute("select base64_encode(?)", [content]).fetchone()[0]
    self.assertEqual(base64_encode('angel'), 'YW5nZWw=')


  
class TestCoverage(unittest.TestCase):                                      
  def test_coverage(self):                                                      
    test_methods = [method for method in dir(TestBase64) if method.startswith('test_')]
    funcs_with_tests = set([x.replace("test_", "") for x in test_methods])
    
    for func in FUNCTIONS:
      self.assertTrue(func in funcs_with_tests, f"{func} does not have corresponding test in {funcs_with_tests}")
    
    for module in MODULES:
      self.assertTrue(module in funcs_with_tests, f"{module} does not have corresponding test in {funcs_with_tests}")

if __name__ == '__main__':
    unittest.main()