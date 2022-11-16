import pygame
import random
from PIL import Image as img

class grid():
	def __init__(self,image_path,image_count,size):
		#images
		self.image_path = image_path #image path
		self.image_count = image_count #count of images
		self.images = [] #array for pygame.images
		self.conections = [] #array for connection

		for x in range(self.image_count):#loads image and conections into array
			self.images.append(pygame.image.load(self.image_path+"/"+str(x)+".png"))
			self.conections.append(self.get_conections(self.image_path+"/"+str(x)+".png"))

		#grid
		self.grid_size = size #size of grid for wfc (grid_size*grid_size)
		self.grid		 = [[0 for i in range(self.grid_size)] for i in range(self.grid_size)] #array for states of grid(integer)
		self.tile_size = int(screen_size/self.grid_size) #image size of one tile
		self.grid_entropy = [[0 for i in range(self.grid_size)] for i in range(self.grid_size)]#store the entropys for every tile
		self.grid_needed  = [[[0,0,0,0] for i in range(self.grid_size)] for i in range(self.grid_size)]#store the needet conections for every tile
		self.grid_images  = [[[] for i in range(self.grid_size)] for i in range(self.grid_size)]#store the posible images for every tile

	def get_conections(self,path): #load the conections
		#load image, size
		img1 = img.open(path)
		w = img1.size[0]
		h = img1.size[1]
		pixels = img1.load()
		return [#pixels of every sites from image
			[pixels[x,0] for x in range(w)],
			[pixels[w-1,y] for y in range(h)],
			[pixels[x,h-1] for x in range(w)],
			[pixels[0,y] for y in range(w)]
		]

	def get_entropy(self): 
		#genarate entropy to self.grid_entropy 
		#and every needet conections into grids_needed
		#and every posibile image into grid_images
		
		self.grid_entropy = [[0 for i in range(self.grid_size)] for i in range(self.grid_size)]
		self.grid_needed  = [[[0,0,0,0] for i in range(self.grid_size)] for i in range(self.grid_size)]
		self.grid_images  = [[[] for i in range(self.grid_size)] for i in range(self.grid_size)]
		
		#main
		for x in range(self.grid_size):
			for y in range(self.grid_size):
				tile = self.grid[x][y]
				
				if (tile): #if there is a tile
					self.grid_entropy[x][y] = 0
				else:
					#genarate grid_needed
					if y!=0:
						Utile = self.grid[x][y-1]#up
						if(Utile):self.grid_needed[x][y][0] = self.conections[Utile-1][2]
					if x != self.grid_size-1:
						Rtile = self.grid[x+1][y]#right
						if(Rtile):self.grid_needed[x][y][1] = self.conections[Rtile-1][3]
					if y != self.grid_size-1:
						Dtile = self.grid[x][y+1]#down
						if(Dtile):self.grid_needed[x][y][2] = self.conections[Dtile-1][0]
					if x != 0:
						Ltile = self.grid[x-1][y]#left
						if(Ltile):self.grid_needed[x][y][3] = self.conections[Ltile-1][1]
					
					#generate grid_entropy and grid_images
					t = self.grid_needed[x][y] #unfinished tile
					for j in self.conections:  #all tiles
						#t == j ignor 0 positions in arrays
						if((t[0]==0 or t[0] == j[0])
						and(t[1]==0 or t[1] == j[1])
						and(t[2]==0 or t[2] == j[2])
						and(t[3]==0 or t[3] == j[3])):
							self.grid_entropy[x][y] += 1
							self.grid_images[x][y].append(self.conections.index(j))

	def draw(self):#draw every tile into a pygame window
		for x in range(self.grid_size):
			for y in range(self.grid_size):
				if(self.grid[x][y]):
					screen.blit(
						pygame.transform.scale(self.images[self.grid[x][y]-1],
							(self.tile_size,self.tile_size)),
							(self.tile_size*x,self.tile_size*y)
					)
	
	def update(self):#generate a new tile of the grid
		self.get_entropy()
		
		#searching for the lowest entropy except of 0
		min_entropy = self.image_count+1#stores the min entropy
		for x in self.grid_entropy: 
			for y in x:
				if y!=0 and y<min_entropy:
					min_entropy = y
					break
		if min_entropy == self.image_count+1:loop = False #if there is nothing to generate:grid is finished, while False
		else:
			#select random right tile
			select = True
			cx = 0#counter
			while select:
				cx += 1
				#x and y of tile
				x = random.randint(0,self.grid_size-1)
				y = random.randint(0,self.grid_size-1)
				if(self.grid_entropy[x][y]==min_entropy):#if the entropy of tile == min entropy:generate tile
					select = False
			self.grid[x][y] = random.choice(self.grid_images[x][y])+1


#circuit example
if __name__ == "__main__":
	screen_size = 800
	screen = pygame.display.set_mode([screen_size]*2)
	
	test = grid("circuit2",32,20)
	while True:
		test.update()
		test.draw()
		pygame.display.update()
