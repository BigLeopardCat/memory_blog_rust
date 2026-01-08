import os

sass_path = "src/frontHome/Head/index.sass"

if os.path.exists(sass_path):
    with open(sass_path, "r", encoding="utf-8") as f:
        content = f.read()

    # Normalize newlines
    content = content.replace('\r\n', '\n')

    # Previous attempts might have left the style inconsistent or not what the user wants.
    # User complaints: "So big", "Circle of blank".
    # User request similar to original.
    
    # Original (as found previously):
    # .theme-btn
    #    width: 40px
    #    height: 40px
    #    ...

    # BUT, because of  on , this 40px becomes 80px.
    # If the user thinks 80px is "So big", then the original code WAS big, or the user's environment behaves differently.
    
    # However, the user says "Blue button around a circle of blank".
    # This implies the button is maybe trying to fill a space but failing?
    
    # Let's try to make the button SMALLER and REMOVE possible spacing.
    # And position it absolutely relative to the Avatar? 
    # Currently it is in  which is absolute .
    
    # I will try to update it to be rectangular (strip) as requested before complaining about "original", 
    # BUT small enough to look good when scaled.
    # OR, maybe the user wants it to NOT be scaled?
    # If it is inside homeLogo, it MUST be scaled.
    
    # Let's try 40px width, 20px height. Rectangular strip.
    # And ensure no padding/margin issues.
    
    target_block = """    .theme-btn
      width: 40px
      height: 40px
      line-height: 40px
      background: #ace0f9
      color: #fff
      border-radius: 10px
      font-size: 12px
      cursor: pointer
      text-align: center
      transition: all 0.3s
      font-weight: 600
      box-shadow: 0 2px 5px rgba(0,0,0,0.15)
      padding: 0
      display: flex
      justify-content: center
      align-items: center
      &:hover
        background: #517498
        transform: scale(1.05)"""

    # If I change this to a smaller rectangle, say 60px width, 24px height.
    # And remove line-height to let flex handle it.
    
    new_block = """    .theme-btn
      width: 60px
      height: 24px
      background: #ace0f9
      color: #fff
      border-radius: 4px
      font-size: 10px
      cursor: pointer
      text-align: center
      transition: all 0.3s
      font-weight: 600
      box-shadow: none
      padding: 0
      display: flex
      justify-content: center
      align-items: center
      &:hover
        background: #517498"""

    # But wait, User said: "Why is it so big? Shape involves... blue button around a circle of blank".
    # This description "blue button around a circle of blank" sounds like the button is a RING?
    # Or maybe the button is inside a white box?
    
    # If I look at the HTML:
    # <div className="loginCard" ... gap: '5px'>
    #    <div className="theme-btn">Login</div>
    # </div>
    
    # If loginCard has no background, where is the white coming from?
    # Maybe the text color inside?
    
    # Let's stick to the "Restore original" request but modify it slightly to be "strip rectangle" as hinted by "Shape is wrong".
    # User said: "Go find his source code... button style use his original".
    # I did that. And user said "Why so big?".
    
    # Maybe the "Original" I found was 40x40.
    # Let's try 40px width, 20px height (Strip). 
    # And I will explicitly set background to transparent for parent just in case.
    
    # Actually, let's keep the user's latest complaint in mind "Why so big".
    # I'll reduce the size drastically.
    
    final_block = """    .theme-btn
      width: 50px
      height: 20px
      line-height: 20px
      background: #ace0f9
      color: #fff
      border-radius: 5px
      font-size: 12px
      cursor: pointer
      text-align: center
      transition: all 0.3s
      font-weight: 600
      box-shadow: none
      padding: 0
      display: flex
      justify-content: center
      align-items: center
      &:hover
        background: #517498"""

    if target_block.strip() in content:
        new_content = content.replace(target_block.strip(), final_block.strip())
        with open(sass_path, "w", encoding="utf-8") as f:
            f.write(new_content)
        print("Updated .theme-btn to smaller strip")
    else:
        print("Could not find current block to update")
        # debug
        print(content.find(".theme-btn"))

else:
    print("File not found")
