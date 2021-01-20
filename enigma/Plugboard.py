class Plugboard:
    def __init__(self, couples, text):
        self.list = couples
        if len(self.list) - 1 < 10:
            self.dict = {}
            self.dictify()
            self.text = text
        else:
            raise Exception("too many couples of letters ("+str(len(self.list))+")")
        
    def dictify(self):
        for c in self.list:
            if not self.dict.get(c[0]):
                self.dict[c[0]] = c[1]
                self.dict[c[1]] = c[0]
            else:
                self.dict = {}
                raise Exception("Duplicated letter", c[0])
    
    def apply(self):
        new_text = []
        for c in self.text:
            if self.dict.get(c):
                new_text.append(self.dict[c])
            else:
                new_text.append(c)
        self.text = "".join(new_text)
        return self.text
