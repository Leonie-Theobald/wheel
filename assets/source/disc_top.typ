#import "@preview/cetz:0.5.3"

#set page(
  width: auto,
  height: auto,
  margin: 0pt,
  fill: none, // transparent background
)

#let TEETH = 8
#let TEETH_HEIGHT = 1cm
// For full circle, we need TEETH number of steps
#let ROTATION_STEP = 2 / TEETH * calc.pi * 1rad // 1 / TEETH * 2 * calc.pi 

#let make_points_for_wheel(r_inner) = {
  let points_for_one_teeth = ()
  // first point
  let x = 0
  let y = r_inner + TEETH_HEIGHT
  points_for_one_teeth.push((x, y))

  // second point
  let x = (r_inner + TEETH_HEIGHT) * calc.sin(0.5 * ROTATION_STEP)
  let y = (r_inner + TEETH_HEIGHT) * calc.cos(0.5 * ROTATION_STEP)
  points_for_one_teeth.push((x, y))

  // third point
  let x = r_inner * calc.sin(0.5 * ROTATION_STEP)
  let y = r_inner * calc.cos(0.5 * ROTATION_STEP)
  points_for_one_teeth.push((x, y))

  // forth point
  let x = r_inner * calc.sin(ROTATION_STEP)
  let y = r_inner * calc.cos(ROTATION_STEP)
  points_for_one_teeth.push((x, y))

  // fifth point
  let x = (r_inner + TEETH_HEIGHT) * calc.sin(ROTATION_STEP)
  let y = (r_inner + TEETH_HEIGHT) * calc.cos(ROTATION_STEP)
  points_for_one_teeth.push((x, y))

  points_for_one_teeth
}

#let draw_wheel(points_for_one_teeth, r_inner, numbers_ring, numbers_half_ring) = {
  let radius_text_ring = r_inner - 0.5 * TEETH_HEIGHT
  let radius_text_half_ring = r_inner + 0.5 * TEETH_HEIGHT

  cetz.canvas({
    import cetz.draw: line, content, rect, circle, group

    // draw teeth
    for i in range(0, TEETH) {
      line(..points_for_one_teeth, close: false, stroke: black)
      cetz.draw.rotate(z: ROTATION_STEP)
    }

    // go to correct starting point for numbers
    cetz.draw.rotate(z: -0.25 * ROTATION_STEP)

    // draw numbers
    for i in range(0, 2 * TEETH) {
      // number on ring
      content(
        (0, radius_text_ring),
        rotate(
          0.25 * ROTATION_STEP - i * 0.5 * ROTATION_STEP
        )[#text(
          str(numbers_ring.at(i)),
          fill: black
        )],
        anchor: "center",
      )

      // number on half ring only on every second position
      if calc.rem(i, 2) == 0 {
        content(
          (0, radius_text_half_ring),
          rotate(
            0.25 * ROTATION_STEP - i * 0.5 * ROTATION_STEP
          )[#text(
            str(numbers_half_ring.at(calc.floor(i / 2))), 
            fill: black
          )],
          anchor: "center",
        )
      }

      cetz.draw.rotate(z: 0.5 * ROTATION_STEP)
    }
  })

}

// first disc
#let r_inner_1 = 4cm
#let numbers_ring_1 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)
#let numbers_half_ring_1 = (10, 20, 30, 40, 50, 60, 70, 80)

// second disc
#let r_inner_2 = r_inner_1 + 2 * TEETH_HEIGHT
#let numbers_ring_2 = (10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160)
#let numbers_half_ring_2 = (1, 2, 3, 4, 5, 6, 7, 8)

// third disc
#let r_inner_3 = r_inner_2 + 2 * TEETH_HEIGHT
#let numbers_ring_3 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)
#let numbers_half_ring_3 = (10, 20, 30, 40, 50, 60, 70, 80)

// fourth disc
#let r_inner_4 = r_inner_3 + 2 * TEETH_HEIGHT
#let numbers_ring_4 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)
#let numbers_half_ring_4 = (10, 20, 30, 40, 50, 60, 70, 80)

// draw discs
#let points = make_points_for_wheel(r_inner_1)
#draw_wheel(points, r_inner_1, numbers_ring_1, numbers_half_ring_1)

#pagebreak()
#let points = make_points_for_wheel(r_inner_2)
#draw_wheel(points, r_inner_2, numbers_ring_2, numbers_half_ring_2)

#pagebreak()
#let points = make_points_for_wheel(r_inner_3)
#draw_wheel(points, r_inner_2, numbers_ring_3, numbers_half_ring_3)

#pagebreak()
#let points = make_points_for_wheel(r_inner_4)
#draw_wheel(points, r_inner_2, numbers_ring_4, numbers_half_ring_4)
