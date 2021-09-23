#!/usr/bin/env python3

import argparse
import random
from pathlib import Path
import math
from typing import Callable


from antbinding import Recording, Pose, World


def populate(world: World):
    def rnd_pose():
        x = (-0.5 + random.random())*world.map.width
        y = (-0.5 + random.random())*world.map.height
        rotation = math.pi*2.*random.random()
        return Pose(x=x, y=y, rotation=rotation)

    def place(factory: Callable, n: int, max_tries: int = None):
        max_tries = max_tries or n*3
        while n > 0:
            max_tries -= 1
            if max_tries < 0:
                raise RuntimeError(
                    f'Could not place {n} times stuff on the board')
            try:
                factory(rnd_pose())
                n -= 1
            except LookupError:
                pass

    place(lambda pose: world.add_ant_hill_mirrored(pose, 0), 1)
    place(lambda pose: world.add_sugar_hill_mirrored(pose), 8)
    place(lambda pose: world.add_raspberry_mirrored(pose), 20)


def main(output: Path, width: float, height: float, **kwargs):
    recording = Recording()
    world = World(width=width, height=height)
    populate(world)
    recording.map = world.map

    recording.add_frame(world.snapshot())
    output.parent.mkdir(parents=True, exist_ok=True)
    recording.dump(str(output))


if __name__ == '__main__':
    parser = argparse.ArgumentParser('Simulation')
    parser.add_argument('--output', type=Path)
    parser.add_argument('--width', type=float, default=64.)
    parser.add_argument('--height', type=float, default=64.)

    args = parser.parse_args()
    main(**vars(args))
