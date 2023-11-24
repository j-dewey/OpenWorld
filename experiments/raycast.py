import pygame as pg

VOXELS_ACROSS = 16
VOXELS_UP = 12
VOXEL_TO_PIXEL = 50
PIXEL_TO_VOXEL =  1 / 100

SCRN_WIDTH = VOXELS_ACROSS*VOXEL_TO_PIXEL
SCRN_HEIGHT = VOXELS_UP*VOXEL_TO_PIXEL

def draw_arrow(surf, color, rear, tip, tip_height, tip_width):
    pg.draw.aaline(surf, color, rear, tip, 5)
    line_vector = pg.Vector2([tip[0]-rear[0], tip[1]-rear[1]])
    lv_normal = list(line_vector.normalize())
    rear_vec = pg.Vector2(rear)
    line_base_inter = line_vector.normalize() * tip_height
    change_right = pg.Vector2([lv_normal[0], -lv_normal[1]])
    change_left = pg.Vector2([-lv_normal[0], lv_normal[1]])
    pg.draw.polygon(surf, color, [
        tip, rear_vec + line_base_inter + change_right, rear_vec + line_base_inter + change_left
    ])

def move_point():
    global position
    position = pg.mouse.get_pos()

def set_vector():
    global movement
    mc = pg.mouse.get_pos()
    movement = [ mc[0]-position[0], mc[1]-position[1] ]

if __name__ == '__main__':
    pg.init()
    win = pg.display.set_mode((SCRN_WIDTH, SCRN_HEIGHT))

    voxel_cover = pg.Surface((SCRN_WIDTH,SCRN_HEIGHT))
    voxel_cover.fill((122, 122, 122))
    for x in range(VOXELS_ACROSS):
        pg.draw.line( voxel_cover, (0,0,0), [x*VOXEL_TO_PIXEL, 0], [x*VOXEL_TO_PIXEL, SCRN_HEIGHT] )
    for y in range(VOXELS_UP):
        pg.draw.line( voxel_cover, (0,0,0), [0, y*VOXEL_TO_PIXEL], [SCRN_WIDTH, y*VOXEL_TO_PIXEL] )


    mode = 0
    modes = [move_point, 'cast ray']
    # where ray will be cast from
    position = [0, 0]
    movement = [10, 10]

    while True:
        for ev in pg.event.get():
            if ev.type == pg.QUIT or (ev.type == pg.KEYDOWN and ev.char == 'e'):
                quit()
            elif ev.type == pg.KEYDOWN and ev.char == 'm':
                mode = int(not bool(mode))
            elif ev.type == pg.MOUSEBUTTONDOWN:
                modes[mode]()
        
        win.fill((255, 255, 255))
        win.blit(voxel_cover, (0,0))
        pg.draw.circle(win, (255, 0, 0), position, 5)
        draw_arrow(win, (0, 0, 255), position, [position[0]+movement[0], position[1]+movement[1]], 10, 20)
        pg.display.flip()
