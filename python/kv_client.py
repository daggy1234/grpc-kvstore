import logging
import grpc
import kvstore_pb2
import kvstore_pb2_grpc


def run():
	print("run")
	with grpc.insecure_channel('localhost:50051') as channel:
		clt = kvstore_pb2_grpc.CacheServiceStub(channel)
		print("Setting Value, Awaiting Input")
		while True:
			print("Enter Option:\n1)GET\n2)PUT\n3)DELETE\n4)Get All")
			opt = int(input("Enter item: "))
			if opt <= 0 or opt >= 5:
				break
			if opt == 1:
				kinp = input("ENter Key: ")
				p_res = clt.Get(kvstore_pb2.GetParams(key=kinp))
				print(p_res)
			elif opt == 2:
				kinp = input("ENter Key: ")
				vinp = input("Enter Value: ")
				p_res = clt.Put(kvstore_pb2.Item(key=kinp,value=vinp))
				print(p_res)
			elif opt == 3:
				kinp = input("ENter Key: ")
				p_res = clt.Delete(kvstore_pb2.GetParams(key=kinp))
				if p_res.success:
					print("Del Done")
				else:
					print("Del Failed")	
			elif opt == 4:
				all_itms = clt.GetAllItems(kvstore_pb2.AllItemsParams())
				for itm in all_itms:
					print(itm)
				# mst = []
				# for itm in all_itms:
				# 	mst.append((itm.key, itm.value))
				# print(mst)



if __name__ == '__main__':
    logging.basicConfig()
    run()