#!/usr/bin/env python3

import argparse
import random
from pathlib import Path

from antbinding import Recording, Frame, Pose, AntHill, SugarHill


class AntColony:
    def __init__(self, name, x, y):
        self.name = name
        self.ant_hill = AntHill(x, y)
        self.ants = []


class World:
    HILL_RADIUS = 3.
    RASPBERRY_RADIUS = 1.

    def __init__(self, team_a_agent, team_b_agent):
        '''
        Every spawned object is mirrored
        '''
        INITIAL_RASPBERRIES_COUNT = 15
        INITIAL_SUGAR_HILLS_COUNT = 5

        def mirror(pos, ctor):
            x, y = pos
            return [ctor(x=x, y=y), ctor(x=-x, y=-y)]

        # from -width <-> +width
        self.world_size = {'width': 64, 'height': 64}
        self.raspberries = []
        self.sugar_hills = []

        ant_hill_x = 64. + random.random()*32.
        ant_hill_y = 64. + random.random()*32.
        self.team_a = AntColony('team A', x=ant_hill_x, y=ant_hill_y)
        self.team_b = AntColony('team B', x=-ant_hill_x, y=-ant_hill_y)
        for _ in range(INITIAL_RASPBERRIES_COUNT):
            self.raspberries += [r for r in mirror(
                self._find_free_space(self.RASPBERRY_RADIUS), Pose)]

        for _ in range(INITIAL_SUGAR_HILLS_COUNT):
            self.sugar_hills += [r for r in mirror(
                self._find_free_space(self.HILL_RADIUS), SugarHill)]

        self.team_a_agent = team_a_agent
        self.team_b_agent = team_b_agent

    def _check_collide(self, pos, radius, mirror=True):
        x, y = pos

        def square_dis(other_pose, flip=False):
            if flip:
                dx = -x-other_pose.x
                dy = -y-other_pose.y
            else:
                dx = x-other_pose.x
                dy = y-other_pose.y

            return dx*dx+dy*dy

        def min_square_dis(other_pose):
            if not mirror:
                return square_dis(other_pose)
            else:
                return min(
                    square_dis(other_pose, flip=False),
                    square_dis(other_pose, flip=True),
                )

        if mirror:
            d = square_dis(Pose(-x, -y))
            if d < (2.*radius)**2:
                return True

        for raspberry in self.raspberries:
            d = min_square_dis(raspberry)
            if d < (radius+self.RASPBERRY_RADIUS)**2:
                return True

        for hill in self.sugar_hills:
            d = min_square_dis(hill.pose)
            if d < (radius+self.HILL_RADIUS)**2:
                return True
        for hill in [self.team_a.ant_hill, self.team_b.ant_hill]:
            d = min_square_dis(hill.pose)
            if d < (radius+self.HILL_RADIUS)**2:
                return True

        for ant in self.team_a.ants + self.team_b.ants:
            d = min_square_dis(ant.pose)
            if d < (radius+self.HILL_RADIUS)**2:
                return True

    def _find_free_space(self, radius):
        MAX_TRIES = 100
        for _ in range(MAX_TRIES):
            # Proposal
            x = (-1. + 2.*random.random())*self.world_size['width']
            y = (-1. + 2.*random.random())*self.world_size['height']

            # Validate
            if not self._check_collide((x, y), radius):
                return x, y

        raise TimeoutError(f'Could not find a free space')

    def next_step(self):
        pass

    def snapshot(self):
        frame = Frame()
        # Ants
        for ant in self.team_a.ants:
            # Todo add team
            frame.add_ant(ant)
        for ant in self.team_b.ants:
            # Todo add team
            frame.add_ant(ant)

        # Ant hills
        # Todo add team
        frame.add_anthill(self.team_a.ant_hill)
        frame.add_anthill(self.team_b.ant_hill)

        # Raspberries
        for raspberry in self.raspberries:
            frame.add_raspberry(raspberry)

        # sugar hills
        for sugar_hill in self.sugar_hills:
            # Add volume
            frame.add_sugar_hill(sugar_hill)

        return frame


def main(filename: Path, **kwargs):
    recording = Recording()
    world = World(team_a_agent=None, team_b_agent=None)
    recording.map.width = world.world_size['width']
    recording.map.height = world.world_size['height']

    recording.add_frame(world.snapshot())
    filename.parent.mkdir(parents=True, exist_ok=True)
    recording.dump(str(filename))


if __name__ == '__main__':
    parser = argparse.ArgumentParser('Simulation')
    parser.add_argument('--filename', type=Path)

    args = parser.parse_args()
    main(**vars(args))
