
def cell_update(cell)
  next_cell = Array.new()
  for i in (0..cell.size-1)
    cell_part = Array.new()
    for j in (0..cell[i].size-1)
      cell_part.push(0)
    end
    next_cell.push(cell_part)
  end

  for i in (0..cell.size - 1)
    for j in (0..cell[i].size-1)
      ip = (i+1) >= cell   .size ? 0:i+1
      jp = (j+1) >= cell[i].size ? 0:j+1
      count = 0
      count += cell[ip] [j]   + cell[i]  [jp]
      count += cell[i-1][j]   + cell[i]  [j-1]
      count += cell[i-1][j-1] + cell[ip] [jp]
      count += cell[ip] [j-1] + cell[i-1][jp]
      if cell[i][j] == 1
        case count
        when 2, 3
          next_cell[i][j] = 1
        end
      elsif count == 3
        next_cell[i][j] = 1
      elsif rand(cell.size*cell[0].size*10) < 1 # 10 frameに一回突然発生
        next_cell[i][j] = 1
      end
    end
  end
  next_cell
end

def print_cell(cell)
  cell.each{ |cell_part| 
    cell_part.each { |value|
      if value == 1
        print "■ "
      else
        print "□ "
      end
    }
    print "\n"
  }
end

def main(x:40, y:32, seed:0)
  srand(seed)
  cell = Array.new()
  for i in (0..y-1)
    cell_part = Array.new()
    for j in (0..x-1)
      cell_part.push(rand(2))
    end
    cell.push(cell_part)
  end
  while true
    print_cell(cell)
    sleep(0.18)
    cell = cell_update(cell)
    system("clear")
  end
end

main(seed:ARGV[0].to_i)
