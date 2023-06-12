import grpc
import kvstore_pb2
import kvstore_pb2_grpc

from concurrent import futures
import logging
import math
import time

from kvstore_pb2 import GetParams, Item

class KvMaster(kvstore_pb2_grpc.CacheServiceServicer):

	def __init__(self) -> None:
		self.db = {}

	def Put(self, request: Item, context):
		self.db[request.key] = request.value
		print(self.db)
		return kvstore_pb2.Item(key=request.key,value=request.value)

	def Get(self, request: GetParams, context):
		out = self.db.get(request.key)
		if out is None:
			out = ""
		print(self.db)
		return kvstore_pb2.Item(key=request.key,value=out)

	def Delete(self, request: Item, context):
		exists = self.db.get(request.key)
		if exists is None:
			return kvstore_pb2.Success(success=False)
		del self.db[request.key]
		print(self.db)
		return kvstore_pb2.Success(success=True)

	def GetAllItems(self, request, context):
		for k,v in self.db.items():
			yield kvstore_pb2.Item(key=k,value=v)

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    kvstore_pb2_grpc.add_CacheServiceServicer_to_server(KvMaster(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    server.wait_for_termination()


if __name__ == '__main__':
    logging.basicConfig()
    print("Running")
    serve()