class MFSCommunicationError(RuntimeError):
	def __init__(self, message=""):
		super().__init__("PolarisFS communication error" + (": " + message if message else ""))
