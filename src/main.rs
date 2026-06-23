mod tree_by_levels;
use crate::tree_by_levels::Node;
use crate::tree_by_levels::tree_by_levels;

fn main() {
    let first_root = Node::new(45).right(
        Node::new(35)
            .left(
                Node::new(38).right(
                    Node::new(40)
                        .left(Node::new(1).left(Node::new(20)))
                        .right(Node::new(44).left(Node::new(38)).right(Node::new(41))),
                ),
            )
            .right(
                Node::new(3).right(
                    Node::new(23)
                        .left(Node::new(2).left(Node::new(46)).right(Node::new(2)))
                        .right(Node::new(36)),
                ),
            ),
    );
    // mine:    `[45, 35, 38, 3, 40, 1, 44, 20, 38, 41, 23, 2, 36, 46, 2]`,
    // correct: `[45, 35, 38, 3, 40, 23, 1, 44, 2, 36, 20, 38, 41, 46, 2]`:

    let _second_root = Node::new(141).left(
        Node::new(659).right(
            Node::new(50)
                .left(
                    Node::new(171).left(
                        Node::new(270).right(
                            Node::new(560)
                                .left(Node::new(126).left(Node::new(327)).right(Node::new(512)))
                                .right(Node::new(367).left(Node::new(421)).right(Node::new(976))),
                        ),
                    ),
                )
                .right(
                    Node::new(802).right(
                        Node::new(937)
                            .left(Node::new(858).left(Node::new(146)).right(Node::new(422)))
                            .right(Node::new(44).left(Node::new(726))),
                    ),
                ),
        ),
    );

    // mine:    `[141, 659, 50, 171, 802, 270, 560, 126, 367, 327, 512, 421, 976, 937, 858, 44, 146, 422, 726]`,
    // correct: `[141, 659, 50, 171, 802, 270, 937, 560, 858, 44, 126, 367, 146, 422, 726, 327, 512, 421, 976]`

    println!("tree_by_levels: {:?}", tree_by_levels(&first_root));
}
